use super::media_stream_info::{MediaStreamInfo, StreamType};

pub(super) struct VideoStreamInfo {
    stream_connection_id: String,
}

impl MediaStreamInfo for VideoStreamInfo {
    fn stream_type(&self) -> StreamType {
        StreamType::Video
    }
}

impl VideoStreamInfo {
    pub fn new(id: String) -> Self {
        Self {
            stream_connection_id: id,
        }
    }

    pub fn get_stream_connection_id(&self) -> &str {
        &self.stream_connection_id
    }
}
