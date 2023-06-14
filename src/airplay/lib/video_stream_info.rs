#[derive(Debug)]
pub struct VideoStreamInfo {
    stream_connection_id: String,
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
