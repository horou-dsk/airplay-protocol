use crate::utils::encode_hex;

use super::{
    audio_stream_info::{AudioFormat, AudioStreamInfo, CompressionType},
    media_stream_info::MediaStreamInfo,
    video_stream_info::VideoStreamInfo,
};

#[derive(Default)]
pub(super) struct Rtsp {
    ekey: Vec<u8>,
    eiv: Vec<u8>,
    stream_connection_id: String,
}

impl Rtsp {
    pub fn get_stream_connection_id(&self) -> String {
        self.stream_connection_id.clone()
    }

    pub fn get_ekey(&self) -> &Vec<u8> {
        &self.ekey
    }

    pub fn get_eiv(&self) -> &Vec<u8> {
        &self.eiv
    }

    pub fn setup(&mut self, data: &[u8]) -> Option<MediaStreamInfo> {
        let setup: plist::Dictionary = plist::from_bytes(data).unwrap();
        if setup.contains_key("ekey") || setup.contains_key("eiv") {
            self.ekey = setup["ekey"].as_data().map(|v| v.to_vec()).unwrap();
            self.eiv = setup["eiv"].as_data().map(|v| v.to_vec()).unwrap();
            log::info!(
                "Encrypted AES key: {}, iv: {}",
                encode_hex(&self.ekey),
                encode_hex(&self.eiv)
            );
            None
        } else if setup.contains_key("streams") {
            log::debug!("RTSP SETUP streams:\n{:#?}", setup);
            self.get_media_stream_info(setup)
        } else {
            log::error!("Unknown RTSP setup content\n{:#?}", setup);
            None
        }
    }

    pub fn teardown(&mut self, data: &[u8]) -> Option<MediaStreamInfo> {
        let teardown: plist::Dictionary = plist::from_bytes(data).unwrap();
        log::debug!("RTSP TEARDOWN streams:\n{:#?}", teardown);
        if teardown.contains_key("streams") {
            self.get_media_stream_info(teardown)
        } else {
            None
        }
    }

    pub fn get_media_stream_info(&mut self, request: plist::Dictionary) -> Option<MediaStreamInfo> {
        let streams = request.get("streams").unwrap().as_array().unwrap();
        if streams.len() > 1 {
            log::warn!("Request contains more than one stream info");
        }
        let stream = streams[0].as_dictionary().unwrap();
        let ty = stream["type"].as_signed_integer().unwrap();
        match ty {
            110 => {
                if stream.contains_key("streamConnectionID") {
                    self.stream_connection_id = stream["streamConnectionID"]
                        .as_signed_integer()
                        .map(|n| n as u64)
                        .unwrap()
                        .to_string();
                }
                Some(MediaStreamInfo::Video(VideoStreamInfo::new(
                    self.stream_connection_id.clone(),
                )))
            }
            96 => {
                let mut builder = AudioStreamInfo::builder();
                if let Some(ct) = stream.get("ct") {
                    builder.compression_type(CompressionType::from_code(
                        ct.as_signed_integer().unwrap(),
                    ));
                }
                if let Some(audio_format) = stream.get("audioFormat") {
                    builder.audio_format(AudioFormat::from_code(
                        audio_format.as_signed_integer().unwrap(),
                    ));
                }
                if let Some(spf) = stream.get("spf") {
                    builder.samples_per_frame(spf.as_unsigned_integer().unwrap());
                }
                Some(MediaStreamInfo::Audio(builder.build()))
            }
            _ => {
                log::error!("Unknown stream type: {}", ty);
                None
            }
        }
    }
}
