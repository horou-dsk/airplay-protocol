use bytes::Bytes;
use ed25519_dalek::PUBLIC_KEY_LENGTH;

use self::{
    fairplay::FairPlay, fairplay_audio_decryptor::FairPlayAudioDecryptor,
    fairplay_video_decryptor::FairPlayVideoDecryptor, media_stream_info::MediaStreamInfo,
    pairing::Pairing,
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
pub mod fairplay_audio_decryptor;
pub mod fairplay_video_decryptor;
mod hand_garble;
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

    pub fn audio_decryptor(&self) -> FairPlayAudioDecryptor {
        FairPlayAudioDecryptor::new(
            self.get_fairplay_aes_key(),
            self.rtsp.get_eiv(),
            self.pairing.get_shared_secret(),
        )
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use tp_macro::jb_to_rb;

    use crate::airplay::lib::fairplay_video_decryptor::FairPlayVideoDecryptor;

    use super::AirPlay;

    #[test]
    fn fair_play_test() {
        let mut airplay = AirPlay::default();
        let steup1_buf = jb_to_rb!([70, 80, 76, 89, 3, 1, 1, 0, 0, 0, 0, 4, 2, 0, 0, -69]);
        let resp1 = airplay.fairplay_setup(&steup1_buf);

        assert_eq!(
            Some(Bytes::from_static(&jb_to_rb!([
                70, 80, 76, 89, 3, 1, 2, 0, 0, 0, 0, -126, 2, 0, 15, -97, 63, -98, 10, 37, 33, -37,
                -33, 49, 42, -78, -65, -78, -98, -115, 35, 43, 99, 118, -88, -56, 24, 112, 29, 34,
                -82, -109, -40, 39, 55, -2, -81, -99, -76, -3, -12, 28, 45, -70, -99, 31, 73, -54,
                -86, -65, 101, -111, -84, 31, 123, -58, -9, -32, 102, 61, 33, -81, -32, 21, 101,
                -107, 62, -85, -127, -12, 24, -50, -19, 9, 90, -37, 124, 61, 14, 37, 73, 9, -89,
                -104, 49, -44, -100, 57, -126, -105, 52, 52, -6, -53, 66, -58, 58, 28, -39, 17,
                -90, -2, -108, 26, -118, 109, 74, 116, 59, 70, -61, -89, 100, -98, 68, -57, -119,
                85, -28, -99, -127, 85, 0, -107, 73, -60, -30, -9, -93, -10, -43, -70
            ]))),
            resp1
        );

        let setup2_buf = jb_to_rb!([
            70, 80, 76, 89, 3, 1, 3, 0, 0, 0, 0, -104, 0, -113, 26, -100, -40, -92, -10, 52, 109,
            20, 120, 6, -62, -67, -118, 75, -47, -71, -109, -45, -61, 106, -95, 1, 36, -104, -7,
            78, -1, -13, 70, 123, -49, 27, 49, -104, 98, 92, -94, 69, -114, 62, -48, 30, -35, 53,
            -25, 41, 53, 125, -7, 75, -128, -51, 10, -50, 35, 84, -42, -116, -29, 127, 94, 24, -16,
            -49, -46, 109, 65, 103, 21, 63, -64, -76, 54, 35, 22, 111, 8, -58, 111, -45, 1, 56, 14,
            -80, -98, -97, -115, -24, 59, -46, -82, -57, -92, 1, -15, -5, -67, -13, 46, 10, -43,
            81, -24, 121, 63, -25, -63, 25, 35, 51, -103, -91, 53, 76, -59, 67, 7, 30, -68, -50,
            -32, -84, -123, 34, -82, 27, -85, 51, -44, 65, -60, 120, -11, 99, -50, -3, 66, 117, -5,
            85, 90, 58, -29, 58, -40, -71, -7, -108, -7, -75
        ]);
        let resp2 = airplay.fairplay_setup(&setup2_buf);

        assert_eq!(
            Some(Bytes::from_static(&jb_to_rb!([
                70, 80, 76, 89, 3, 1, 4, 0, 0, 0, 0, 20, -60, 120, -11, 99, -50, -3, 66, 117, -5,
                85, 90, 58, -29, 58, -40, -71, -7, -108, -7, -75
            ]))),
            resp2
        );

        let encrypted_aes_key = jb_to_rb!([
            70, 80, 76, 89, 1, 2, 1, 0, 0, 0, 0, 60, 0, 0, 0, 0, 63, 121, 70, -69, 3, -8, 117, -13,
            83, 72, 105, -51, -11, -43, -1, 17, 0, 0, 0, 16, 24, -109, 13, 105, -32, -125, -73,
            -128, 21, 29, -31, 72, -41, 112, -36, -75, 57, 110, 71, -72, -25, -59, 102, 22, 19,
            -43, 35, 74, -20, 86, 15, 16, 126, 5, 15, -45
        ]);
        let mut setup3 = plist::Dictionary::default();
        setup3.insert(
            "ekey".to_string(),
            plist::Value::Data(encrypted_aes_key.to_vec()),
        );
        setup3.insert(
            "eiv".to_string(),
            plist::Value::Data("91IdM6RTh4keicMei2GfQA==".as_bytes().to_vec()),
        );
        let mut setup3_buf = Vec::new();
        plist::to_writer_binary(&mut setup3_buf, &setup3).unwrap();
        airplay.rstp_setup(&setup3_buf);

        let stream_connection_id = -3907568444900622110i64;

        let mut data_stream = plist::Dictionary::default();
        data_stream.insert("type".to_string(), 110.into());
        data_stream.insert(
            "streamConnectionID".to_string(),
            stream_connection_id.into(),
        );

        let streams = plist::Value::Array(vec![plist::Value::Dictionary(data_stream)]);

        let mut rtsp_setup2 = plist::Dictionary::default();
        rtsp_setup2.insert("streams".to_string(), streams);

        let mut rtsp_setup2_buf = Vec::new();

        plist::to_writer_binary(&mut rtsp_setup2_buf, &rtsp_setup2).unwrap();

        airplay.rstp_setup(&rtsp_setup2_buf);

        let shared_secret = jb_to_rb!([
            -5, -67, -104, 31, 49, 40, -76, 40, -116, 105, 45, -47, 125, -94, 117, -104, -54, -47,
            -50, 6, 122, 1, -38, -114, -88, -85, -128, 2, 116, -119, -90, 123
        ]);

        let mut fairplay_video_decryptor = FairPlayVideoDecryptor::new(
            airplay.get_fairplay_aes_key(),
            shared_secret.to_vec(),
            airplay.rtsp.get_stream_connection_id(),
        );

        let mut payload = include_bytes!("./resources/encrypted_payload").to_vec();
        fairplay_video_decryptor.decrypt(&mut payload);

        let nc_len = i32::from_be_bytes(payload[..4].try_into().unwrap()) as usize;

        assert_eq!(payload.len() - 4, nc_len, "Decrypted payload is corrupted!");
    }
}
