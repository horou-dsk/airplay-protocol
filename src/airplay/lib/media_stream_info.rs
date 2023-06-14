use super::{audio_stream_info::AudioStreamInfo, video_stream_info::VideoStreamInfo};

#[derive(Debug)]
pub enum MediaStreamInfo {
    Audio(AudioStreamInfo),
    Video(VideoStreamInfo),
}
