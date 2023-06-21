use aes::cipher::{KeyIvInit, StreamCipher};
use sha2::{Digest, Sha512};

type Aes128Ctr64BE = ctr::Ctr64BE<aes::Aes128>;

#[derive(Clone)]
pub struct FairPlayVideoDecryptor {
    // aes_key: [u8; 16],
    // shared_secret: Vec<u8>,
    // stream_connection_id: String,
    aes_ctr_decrypt: Aes128Ctr64BE,
    og: [u8; 16],
    next_decrypt_count: usize,
}

impl FairPlayVideoDecryptor {
    pub fn new(aes_key: [u8; 16], shared_secret: Vec<u8>, stream_connection_id: String) -> Self {
        let mut hasher = Sha512::new();
        hasher.update(aes_key);
        hasher.update(&shared_secret);
        let eaes_key = hasher.finalize();

        let skey = format!("AirPlayStreamKey{}", stream_connection_id);
        let mut hasher = Sha512::new();
        hasher.update(skey.as_bytes());
        hasher.update(&eaes_key[..16]);
        let hash1 = hasher.finalize();

        let mut hasher = Sha512::new();
        let siv = format!("AirPlayStreamIV{}", stream_connection_id);
        hasher.update(siv.as_bytes());
        hasher.update(&eaes_key[..16]);

        let hash2 = hasher.finalize();
        let decrypt_aes_key = &hash1[..16];
        let decrypt_aes_iv = &hash2[..16];

        Self {
            // aes_key,
            // shared_secret,
            // stream_connection_id,
            aes_ctr_decrypt: Aes128Ctr64BE::new(decrypt_aes_key.into(), decrypt_aes_iv.into()),
            og: [0; 16],
            next_decrypt_count: 0,
        }
    }

    pub fn decrypt(&mut self, video: &mut [u8]) {
        if self.next_decrypt_count > 0 {
            (0..self.next_decrypt_count).for_each(|i| {
                video[i] ^= self.og[(16 - self.next_decrypt_count) + i];
            });
        }

        let encrypt_len = ((video.len() - self.next_decrypt_count) / 16) * 16;
        self.aes_ctr_decrypt.apply_keystream(
            &mut video[self.next_decrypt_count..self.next_decrypt_count + encrypt_len],
        );
        // System.arraycopy(video, nextDecryptCount, video, nextDecryptCount, encryptlen);

        let rest_len = (video.len() - self.next_decrypt_count) % 16;
        let rest_start = video.len() - rest_len;
        self.next_decrypt_count = 0;
        if rest_len > 0 {
            self.og = [0; 16];
            self.og[..rest_len].copy_from_slice(&video[rest_start..rest_start + rest_len]);
            // System.arraycopy(video, reststart, og, 0, restlen);
            self.aes_ctr_decrypt.apply_keystream(&mut self.og);
            // aesCtrDecrypt.update(og, 0, 16, og, 0);
            video[rest_start..rest_start + rest_len].copy_from_slice(&self.og[..rest_len]);
            // System.arraycopy(og, 0, video, reststart, restlen);
            self.next_decrypt_count = 16 - rest_len;
        }
    }
}
