use crate::utils::encode_hex;

use super::{media_stream_info::MediaStreamInfo, video_stream_info::VideoStreamInfo};

#[derive(Default)]
pub(super) struct Rtsp {
    ekey: Vec<u8>,
    eiv: Vec<u8>,
    stream_connection_id: String,
}

impl Rtsp {
    pub fn get_ekey(&self) -> &Vec<u8> {
        &self.ekey
    }

    pub fn get_eiv(&self) -> &Vec<u8> {
        &self.eiv
    }

    pub fn setup(&mut self, data: &[u8]) -> Option<Box<dyn MediaStreamInfo>> {
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
            todo!()
            // Some()
        } else {
            todo!()
        }
    }

    pub fn get_media_stream_info(
        &mut self,
        request: plist::Dictionary,
    ) -> Box<dyn MediaStreamInfo> {
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
                        .unwrap()
                        .to_string();
                }
                Box::new(VideoStreamInfo::new(self.stream_connection_id.clone()))
            }
            96 => {
                todo!()
            }
            _ => todo!(),
        }
    }
}
