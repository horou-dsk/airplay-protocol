use aes::cipher::generic_array::GenericArray;
use aes::cipher::typenum::U16;
use aes::cipher::{KeyIvInit, StreamCipher};
use aes::Aes128;
use aes_gcm::aead::Aead;
use bytes::{BufMut, Bytes};
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::{constants::X25519_BASEPOINT, MontgomeryPoint};
use ed25519_dalek::{
    Signature, Signer, SigningKey, Verifier, VerifyingKey, PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH,
};
use rand::rngs::OsRng;
use sha2::{Digest, Sha512};

use crate::srp::airsrp::{AirSrp, Handshake, NgType};

// use crate::airplay::property_list;

type Aes128Ctr64BE = ctr::Ctr64BE<aes::Aes128>;

pub(super) struct Pairing {
    keypair: SigningKey,
    ed_theirs: [u8; SECRET_KEY_LENGTH],
    ecdh_ours: [u8; 32],
    ecdh_theirs: Bytes,
    ecdh_secret: [u8; 32],
    pair_verified: bool,
    handshake: Option<Handshake>,
    session_key: Option<Vec<u8>>,
}

impl Default for Pairing {
    fn default() -> Self {
        let mut csprng = rand::rngs::OsRng {};
        let keypair = SigningKey::generate(&mut csprng);
        Self {
            keypair,
            ed_theirs: Default::default(),
            ecdh_ours: Default::default(),
            ecdh_theirs: Default::default(),
            ecdh_secret: Default::default(),
            pair_verified: Default::default(),
            handshake: None,
            session_key: None,
        }
    }
}

impl Pairing {
    pub fn pair_setup_pin(&mut self, data: &[u8]) -> Option<Bytes> {
        let plist_data: plist::Dictionary = plist::from_bytes(data).unwrap();
        log::info!("{:?}", plist_data);
        let mut result = plist::Dictionary::default();
        if plist_data.contains_key("method") && plist_data.contains_key("user") {
            let username = plist_data["user"].as_string().unwrap();
            let air_srp = AirSrp::new(NgType::SrpNg2048, username, "2222");
            let handshake = air_srp.create_salted_verification_key(self.pair_setup());
            let salt = handshake.salt.to_bytes_be().1;
            result.insert(
                "pk".to_string(),
                plist::Value::Data(handshake.pk_B.to_bytes_be().1),
            );
            result.insert("salt".to_string(), plist::Value::Data(salt));
            self.handshake = Some(handshake);
        } else if plist_data.contains_key("pk") && plist_data.contains_key("proof") {
            let a = plist_data["pk"].as_data().unwrap();
            // 缺少验证 M1 是否相等
            // let m = plist_data["proof"].as_data().unwrap();
            if let Some(handshake) = self.handshake.take() {
                let ver = handshake.new_verifier(a);
                result.insert("proof".to_string(), plist::Value::Data(ver.M2.to_vec()));
                self.session_key = Some(ver.session_key);
            }
        } else if plist_data.contains_key("epk") && plist_data.contains_key("authTag") {
            let epk = plist_data["epk"].as_data().unwrap();
            let auth_tag = plist_data["authTag"].as_data().unwrap();
            if let Some((epk, auth_tag)) = self.verify_pin(epk, auth_tag) {
                log::info!("to epk = {:?}\nauthTag = {:?}", epk, auth_tag);
                result.insert("epk".to_string(), plist::Value::Data(epk));
                result.insert("authTag".to_string(), plist::Value::Data(auth_tag));
            }
        }
        if !result.is_empty() {
            let mut writer = bytes::BytesMut::default().writer();
            plist::to_writer_binary(&mut writer, &result).unwrap();
            Some(writer.into_inner().freeze())
        } else {
            None
        }
    }

    pub fn pair_setup(&self) -> [u8; PUBLIC_KEY_LENGTH] {
        self.keypair.verifying_key().to_bytes()
    }

    pub fn pair_verify(&mut self, bytes: &[u8]) -> Option<Bytes> {
        log::info!("{:?}", bytes);
        let flag = bytes[0];
        let bytes = &bytes[4..];
        if flag > 0 {
            let ecdh_theirs = &bytes[..32];
            self.ecdh_theirs = Bytes::copy_from_slice(ecdh_theirs);
            self.ed_theirs = bytes[32..].try_into().unwrap();
            let mut rng = OsRng;
            let private_key = Scalar::random(&mut rng);
            let ecdh_ours = private_key * X25519_BASEPOINT;
            self.ecdh_ours = ecdh_ours.0;
            // let ecdh_theirs = CompressedEdwardsY::from_slice(ecdh_theirs)
            //     .unwrap()
            //     .decompress()
            //     .unwrap();
            let mut pk = [0; 32];
            pk.copy_from_slice(ecdh_theirs);
            let ecdh_theirs = MontgomeryPoint(pk);
            // let ecdh_theirs = CompressedRistretto::from_slice(ecdh_theirs)
            //     .unwrap()
            //     .decompress()
            //     .unwrap();

            // 计算 ECDH 密钥协商
            let ecdh_secret = ecdh_theirs * private_key;
            self.ecdh_secret = ecdh_secret.0;

            let mut cipher = self.init_cipher();

            let mut data_to_sign = [0; 64];
            data_to_sign[..32].copy_from_slice(&self.ecdh_ours);
            data_to_sign[32..].copy_from_slice(ecdh_theirs.as_bytes());
            let signature = self.keypair.sign(&data_to_sign);

            let mut encrypted_signature = signature.to_vec();
            cipher.apply_keystream(&mut encrypted_signature);

            let mut result = Vec::with_capacity(self.ecdh_ours.len() + encrypted_signature.len());
            // log::info!("{} {}", self.ecdh_ours.len(), encrypted_signature.len());
            result.extend_from_slice(&self.ecdh_ours);
            result.extend_from_slice(&encrypted_signature);
            Some(result.into())
        } else {
            let mut cipher = self.init_cipher();
            let mut signature = bytes.to_vec();
            let mut sig_buffer = [0; 64];
            cipher.apply_keystream(&mut sig_buffer);
            cipher.apply_keystream(&mut signature);

            let mut sig_message = sig_buffer;
            sig_message[..32].copy_from_slice(&self.ecdh_theirs);
            sig_message[32..].copy_from_slice(&self.ecdh_ours);

            let signature = Signature::from_slice(&signature).expect("signature error !!!");
            let verifying_key = VerifyingKey::from_bytes(&self.ed_theirs).unwrap();
            // verifying_key.verify(msg, signature)

            let pair_verified = verifying_key.verify(&sig_message, &signature);
            log::info!("{:?}", pair_verified);
            self.pair_verified = pair_verified.is_ok();
            None
        }
    }

    fn init_cipher(&self) -> Aes128Ctr64BE {
        let mut hasher = Sha512::new();
        hasher.update(b"Pair-Verify-AES-Key");
        // log::info!("ecdh_secret = {:?}", self.ecdh_secret);
        hasher.update(self.ecdh_secret);

        let mut shared_secret_sha512_aes_key = [0u8; 16];
        shared_secret_sha512_aes_key.copy_from_slice(&hasher.finalize()[..16]);

        let mut hasher = Sha512::new();
        hasher.update(b"Pair-Verify-AES-IV");
        hasher.update(self.ecdh_secret);

        let mut shared_secret_sha512_aes_iv = [0u8; 16];
        shared_secret_sha512_aes_iv.copy_from_slice(&hasher.finalize()[..16]);

        Aes128Ctr64BE::new(
            &shared_secret_sha512_aes_key.into(),
            &shared_secret_sha512_aes_iv.into(),
        )
    }

    fn verify_pin(&mut self, epk: &[u8], auth_tag: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
        use aes_gcm::{AesGcm, KeyInit};
        type Aes128Gcm = AesGcm<Aes128, U16>;
        if let Some(session_key) = &self.session_key {
            log::info!("session_key = {:?}", session_key);
            let mut hasher = Sha512::new();
            hasher.update(b"Pair-Setup-AES-Key");
            hasher.update(session_key);

            let mut aes_key = [0u8; 16];
            aes_key.copy_from_slice(&hasher.finalize_reset()[..16]);

            hasher.update(b"Pair-Setup-AES-IV");
            hasher.update(session_key);
            let mut aes_iv = [0u8; 16];
            aes_iv.copy_from_slice(&hasher.finalize_reset()[..16]);
            for ivv in aes_iv.iter_mut().rev() {
                *ivv += 1;
                if *ivv != 0 {
                    break;
                }
            }
            // aes_iv[15] += 1;
            let cipher = Aes128Gcm::new(&aes_key.into());

            let iv = GenericArray::from_slice(&aes_iv);
            let data = [epk, auth_tag].concat();
            let result = cipher.decrypt(iv, &*data).unwrap();

            log::info!("decrypted = {:?}", result);

            // let decrypted_key = result.try_into().unwrap();

            // TODO: 无资料，逻辑缺失。

            Some((vec![0; 32], vec![0; 16]))
        } else {
            None
        }
    }

    pub fn get_shared_secret(&self) -> &[u8; 32] {
        &self.ecdh_secret
    }
}
