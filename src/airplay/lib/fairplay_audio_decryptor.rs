use aes::cipher::{generic_array::GenericArray, BlockDecryptMut, KeyIvInit};
use sha2::{Digest, Sha512};

type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

#[derive(Clone)]
pub struct FairPlayAudioDecryptor {
    e_aes_key: [u8; 16],
    aes_iv: Vec<u8>,
}

impl FairPlayAudioDecryptor {
    pub fn new(aes_key: [u8; 16], aes_iv: &[u8], shared_secret: &[u8]) -> Self {
        let mut hasher = Sha512::new();
        hasher.update(aes_key);
        hasher.update(shared_secret);
        let e_aes_key: [u8; 16] = hasher.finalize()[..16].try_into().unwrap();

        log::info!("e_aes_key = {:?}", e_aes_key);
        log::info!("aes_iv = {:?}", aes_iv);

        Self {
            e_aes_key,
            aes_iv: aes_iv.to_vec(),
        }
    }

    pub fn decrypt(&self, audio: &mut [u8]) {
        // log::info!("e_aes_key: {:?}", self.e_aes_key);
        // log::info!("iv: {:?}", self.aes_iv);
        // log::info!("audio_data: {:?}", audio);
        let iv = GenericArray::from_slice(&self.aes_iv);
        let mut aes_cbc_decrypt = Aes128CbcDec::new(&self.e_aes_key.into(), iv);
        for i in 0..(audio.len() / 16) {
            let block_audio = &mut audio[i * 16..(i + 1) * 16];
            aes_cbc_decrypt.decrypt_block_mut(block_audio.into());
        }
    }
}
