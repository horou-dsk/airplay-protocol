use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use sha2::{Digest, Sha512};

type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

#[derive(Clone)]
pub struct FairPlayAudioDecryptor {
    e_aes_key: [u8; 16],
    aes_iv: [u8; 16],
}

impl FairPlayAudioDecryptor {
    pub fn new(aes_key: &[u8], aes_iv: &[u8], shared_secret: &[u8]) -> Self {
        let mut hasher = Sha512::new();
        hasher.update(aes_key);
        hasher.update(shared_secret);
        let e_aes_key: [u8; 16] = hasher.finalize()[..16].try_into().unwrap();

        log::info!("e_aes_key = {:?}", e_aes_key);
        log::info!("aes_iv = {:?}", aes_iv);

        Self {
            e_aes_key,
            aes_iv: aes_iv.try_into().unwrap(),
        }
    }

    pub fn decrypt(&self, audio: &mut [u8]) {
        let mut aes_cbc_decrypt = Aes128CbcDec::new(&self.e_aes_key.into(), &self.aes_iv.into());
        aes_cbc_decrypt.decrypt_block_mut(audio.into());
    }
}
