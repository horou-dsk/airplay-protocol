use std::ops::Deref;

use aes::cipher::{KeyIvInit, StreamCipher};
use bytes::{BufMut, Bytes};
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::{constants::X25519_BASEPOINT, MontgomeryPoint};
use ed25519_dalek::{Signature, Signer, SigningKey, PUBLIC_KEY_LENGTH};
use rand::rngs::OsRng;
use rand::Rng;
use sha2::{Digest, Sha512};
use srp6::*;

use crate::airplay::property_list;

type Aes128Ctr64BE = ctr::Ctr64BE<aes::Aes128>;

type Srp6Air = Srp6<256, 16>;

struct AirSrp(Srp6Air);

impl Default for AirSrp {
    fn default() -> Self {
        Self(Srp6Air::new(
            Generator::from(7),
            "93BE8A2C0FAC7442480A9253539E32A6DE3C3F33D4B7DB4431344F41CAA975E28B626D23E553FCB1450850777ED260D2FFE1FB9816A6ED7164CD76D05733DE4EFA931514D008B7EA8A4BAC45AB7DFD8C346B924E04C37420EEAFCD486159FB49A236DC77B6884FBF3907F0AB8ED789692BA424C81E35A61A38C72EC3A7268B069FCBFBC236AA3167A11E1FD5CD1275021BAC8493CA3AAEBF4AEF685E93A0387C10861F8DB1C500D3DE1823D905EAB421D1E0FD92CEE61F44FF439D07388F1BA56DA112589878D565A199A3C27630DA8FAD31E07EE0A46269B302F215DD972CF9E746867F608DA4DA28A69399708FADC795A6B16276EB6EF5A90636D86DAF03A5"
                .try_into()
                .unwrap()
        ).unwrap())
    }
}

impl Deref for AirSrp {
    type Target = Srp6Air;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(super) struct Pairing {
    keypair: SigningKey,
    ed_theirs: Bytes,
    ecdh_ours: Bytes,
    ecdh_theirs: Bytes,
    ecdh_secret: Bytes,
    pair_verified: bool,
    salt: [u8; 16],
    username: Option<String>,
    proof_verifier: Option<HandshakeProofVerifier>,
}

impl Default for Pairing {
    fn default() -> Self {
        let mut csprng = rand::rngs::OsRng {};
        let keypair = SigningKey::generate(&mut csprng);
        let salt = rand::thread_rng().gen::<[u8; 16]>();
        log::info!(
            "salt = {:?}\npublic_key = {:?}",
            salt,
            keypair.verifying_key().as_bytes()
        );
        Self {
            keypair,
            ed_theirs: Default::default(),
            ecdh_ours: Default::default(),
            ecdh_theirs: Default::default(),
            ecdh_secret: Default::default(),
            pair_verified: Default::default(),
            salt,
            username: None,
            proof_verifier: None,
        }
    }
}

impl Pairing {
    pub fn pair_setup_pin(&mut self, data: &[u8]) -> Option<Bytes> {
        let plist_data: plist::Dictionary = plist::from_bytes(data).unwrap();
        log::info!("{:?}", plist_data);
        let mut result = plist::Dictionary::default();
        if plist_data.contains_key("method") && plist_data.contains_key("user") {
            self.username = plist_data["user"].clone().into_string();
            let username = self.username.as_ref().unwrap().clone();
            let (salt, verifier) = AirSrp::default().generate_new_user_secrets(&username, "2222");
            result.insert("pk".to_string(), plist::Value::Data(verifier.to_vec()));
            result.insert("salt".to_string(), plist::Value::Data(salt.to_vec()));
            let user = UserDetails {
                username,
                salt,
                verifier,
            };
            // let user = mock
            let (_handshake, proof_verifier) = AirSrp::default().start_handshake(&user);
            self.proof_verifier = Some(proof_verifier);
        } else if plist_data.contains_key("pk") && plist_data.contains_key("proof") {
            let a = plist_data["pk"].as_data().unwrap();
            let m = plist_data["proof"].as_data().unwrap();
            let a = PublicKey::from_bytes_le(a);
            let m1 = Proof::from_bytes_le(m);
            let proof = HandshakeProof::<256, 16> { A: a, M1: m1 };
            if let Some(proof_verifier) = self.proof_verifier.take() {
                log::info!("{:?}", proof_verifier.verify_proof(&proof));
            }
            // result.insert(
            //     "proof".to_string(),
            //     plist::Value::Data(property_list::compute_m2(
            //         &self.salt,
            //         plist_data["pk"].as_data().unwrap(),
            //         plist_data["proof"].as_data().unwrap(),
            //     )),
            // );
            result.insert("proof".to_string(), plist::Value::Data(self.srp_m1()));
        } else if plist_data.contains_key("epk") && plist_data.contains_key("authTag") {
            result.insert("epk".to_string(), plist_data["epk"].clone());
            result.insert("authTag".to_string(), plist_data["authTag"].clone());
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
            let ed_theirs = &bytes[32..];
            self.ed_theirs = Bytes::copy_from_slice(ed_theirs);
            let mut rng = OsRng;
            let private_key = Scalar::random(&mut rng);
            let ecdh_ours = private_key * X25519_BASEPOINT;
            self.ecdh_ours = Bytes::copy_from_slice(ecdh_ours.as_bytes());
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
            self.ecdh_secret = Bytes::copy_from_slice(ecdh_secret.as_bytes());

            let mut cipher = self.init_cipher();

            let mut data_to_sign = [0; 64];
            data_to_sign[..32].copy_from_slice(&self.ecdh_ours);
            data_to_sign[32..].copy_from_slice(ecdh_theirs.as_bytes());
            let signature = self.keypair.sign(&data_to_sign);

            let mut encrypted_signature = signature.to_vec();
            cipher.apply_keystream(&mut encrypted_signature);

            let mut result = Vec::with_capacity(self.ecdh_ours.len() + encrypted_signature.len());
            result.extend_from_slice(&self.ecdh_ours);
            result.extend_from_slice(&encrypted_signature);
            Some(result.into())
        } else {
            let mut cipher = self.init_cipher();
            let mut signature = bytes.to_vec();
            cipher.apply_keystream(&mut signature);

            let mut sig_message = [0; 64];
            sig_message[..32].copy_from_slice(&self.ecdh_theirs);
            sig_message[32..].copy_from_slice(&self.ecdh_ours);

            let signature = Signature::from_slice(&signature).expect("signature error !!!");

            self.pair_verified = self.keypair.verify(&sig_message, &signature).is_ok();
            None
        }
    }

    fn init_cipher(&self) -> Aes128Ctr64BE {
        let mut hasher = Sha512::new();
        hasher.update("Pair-Verify-AES-Key".as_bytes());
        log::info!("ecdh_secret = {:?}", self.ecdh_secret);
        hasher.update(&self.ecdh_secret);

        let mut shared_secret_sha512_aes_key = [0u8; 16];
        shared_secret_sha512_aes_key.copy_from_slice(&hasher.finalize()[..16]);

        let mut hasher = Sha512::new();
        hasher.update("Pair-Verify-AES-IV".as_bytes());
        hasher.update(&self.ecdh_secret);

        let mut shared_secret_sha512_aes_iv = [0u8; 16];
        shared_secret_sha512_aes_iv.copy_from_slice(&hasher.finalize()[..16]);

        Aes128Ctr64BE::new(
            &shared_secret_sha512_aes_key.into(),
            &shared_secret_sha512_aes_iv.into(),
        )
    }

    pub fn get_shared_secret(&self) -> &Bytes {
        &self.ecdh_secret
    }

    fn srp_m1(&self) -> Vec<u8> {
        let salt = Salt::from_bytes_be(&self.salt);
        let verifier = PasswordVerifier::from_bytes_be(&self.pair_setup());
        let user = UserDetails {
            username: self.username.as_ref().unwrap().clone(),
            salt,
            verifier,
        };
        // let user = mock
        let (handshake, _proof_verifier) = Srp6_4096::default().start_handshake(&user);
        let password = "2222";
        let (proof, _strong_proof_verifier) = handshake
            .calculate_proof(user.username.as_str(), password)
            .unwrap();
        proof.M1.to_vec()
    }
}
