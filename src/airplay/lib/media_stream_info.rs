pub(super) enum StreamType {
    Audio,
    Video,
}

pub(super) trait MediaStreamInfo {
    fn stream_type(&self) -> StreamType;
}
