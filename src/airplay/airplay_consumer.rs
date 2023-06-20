use std::sync::Arc;

use tokio::sync::RwLock;

use super::lib::{audio_stream_info::AudioStreamInfo, video_stream_info::VideoStreamInfo};

pub trait AirPlayConsumer: Send + Sync {
    fn on_video(&mut self, bytes: Vec<u8>);

    fn on_video_format(&mut self, video_stream_info: VideoStreamInfo);

    fn on_video_src_disconnect(&mut self);

    fn on_audio_format(&mut self, audio_stream_info: AudioStreamInfo);

    fn on_audio(&mut self);
}

pub type ArcAirPlayConsumer = Arc<RwLock<Box<dyn AirPlayConsumer>>>;
