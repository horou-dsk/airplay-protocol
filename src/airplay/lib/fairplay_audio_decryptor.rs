use aes::cipher::{generic_array::GenericArray, BlockDecryptMut, KeyIvInit};
use sha2::{Digest, Sha512};

type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

#[derive(Clone)]
pub struct FairPlayAudioDecryptor {
    aes_cbc_decrypt: Aes128CbcDec,
}

impl FairPlayAudioDecryptor {
    pub fn new(aes_key: [u8; 16], aes_iv: &[u8], shared_secret: &[u8]) -> Self {
        log::info!(
            "\naes_key = {:?}\naes_iv = {:?}\nshared_secret = {:?}",
            aes_key,
            aes_iv,
            shared_secret
        );
        let mut hasher = Sha512::new();
        hasher.update(aes_key);
        hasher.update(shared_secret);
        let e_aes_key: [u8; 16] = hasher.finalize()[..16].try_into().unwrap();

        let iv = GenericArray::from_slice(aes_iv);
        Self {
            aes_cbc_decrypt: Aes128CbcDec::new(&e_aes_key.into(), iv),
        }
    }

    pub fn decrypt(&self, audio: &mut [u8]) {
        let mut aes_cbc_decrypt = self.aes_cbc_decrypt.clone(); //Aes128CbcDec::new(&self.e_aes_key.into(), iv);
        for i in 0..(audio.len() / 16) {
            let block_audio = &mut audio[i * 16..(i + 1) * 16];
            aes_cbc_decrypt.decrypt_block_mut(block_audio.into());
        }
    }
}
