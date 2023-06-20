use bytes::Bytes;
use ed25519_dalek::PUBLIC_KEY_LENGTH;

use self::{
    fairplay::FairPlay, fairplay_video_decryptor::FairPlayVideoDecryptor,
    media_stream_info::MediaStreamInfo, pairing::Pairing,
};

// 将字节数组按照小端字节序转换为 u32
fn to_i32_le(bytes: &[u8]) -> i32 {
    i32::from_le_bytes(bytes.try_into().unwrap())
}

fn get_i32_le(bytes: &[u8], idx: usize) -> i32 {
    to_i32_le(&bytes[idx..idx + 4])
}

fn write_i32_le(bytes: &mut [u8], idx: usize, value: i32) {
    bytes[idx..idx + 4].copy_from_slice(&value.to_le_bytes());
}

pub mod audio_stream_info;
mod fairplay;
pub mod fairplay_video_decryptor;
pub mod hand_garble;
pub mod media_stream_info;
mod modified_md5;
mod omg_hax;
mod omg_hax_const;
mod pairing;
mod rtsp;
mod sap_hash;
pub mod video_stream_info;

#[derive(Default)]
pub struct AirPlay {
    pairing: Pairing,
    fairplay: FairPlay,
    rtsp: rtsp::Rtsp,
    // fairplay_video_decryptor: Option<FairPlayVideoDecryptor>,
}

impl AirPlay {
    pub fn pair_setup_pin(&mut self, data: &[u8]) -> Option<Bytes> {
        self.pairing.pair_setup_pin(data)
    }

    pub fn pair_setup(&self) -> [u8; PUBLIC_KEY_LENGTH] {
        self.pairing.pair_setup()
    }

    pub fn pair_verify(&mut self, data: &[u8]) -> Option<Bytes> {
        self.pairing.pair_verify(data)
    }

    pub fn fairplay_setup(&mut self, data: &[u8]) -> Option<Bytes> {
        self.fairplay.fairplay_setup(data)
    }

    pub fn get_fairplay_aes_key(&self) -> [u8; 16] {
        self.fairplay.decrypt_aes_key(self.rtsp.get_ekey())
    }

    pub fn rstp_setup(&mut self, data: &[u8]) -> Option<MediaStreamInfo> {
        self.rtsp.setup(data)
    }

    pub fn rtsp_teardown(&mut self, data: &[u8]) -> Option<MediaStreamInfo> {
        self.rtsp.teardown(data)
    }

    pub fn video_decryptor(&self) -> FairPlayVideoDecryptor {
        FairPlayVideoDecryptor::new(
            self.get_fairplay_aes_key(),
            self.pairing.get_shared_secret().to_vec(),
            self.rtsp.get_stream_connection_id(),
        )
    }

    // pub fn decrypt_video(&mut self, video: &mut [u8]) {
    //     if let Some(fairplay_video_decryptor) = self.fairplay_video_decryptor.as_mut() {
    //         fairplay_video_decryptor.decrypt(video);
    //     } else {
    //         self.fairplay_video_decryptor = Some(FairPlayVideoDecryptor::new(
    //             self.get_fairplay_aes_key(),
    //             self.pairing.get_shared_secret().to_vec(),
    //             self.rtsp.get_stream_connection_id(),
    //         ))
    //     }
    // }
}
