use aes::cipher::{KeyIvInit, StreamCipher};
use bytes::Bytes;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::{constants::X25519_BASEPOINT, MontgomeryPoint};
use ed25519_dalek::{Signature, Signer, SigningKey, PUBLIC_KEY_LENGTH};
use rand::rngs::OsRng;
use sha2::{Digest, Sha512};

type Aes128Ctr64BE = ctr::Ctr64BE<aes::Aes128>;

pub(super) struct Pairing {
    keypair: SigningKey,
    ed_theirs: Bytes,
    ecdh_ours: Bytes,
    ecdh_theirs: Bytes,
    ecdh_secret: Bytes,
    pair_verified: bool,
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
        }
    }
}

impl Pairing {
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
}
