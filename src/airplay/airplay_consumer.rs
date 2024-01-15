use std::sync::Arc;

use super::{
    lib::{audio_stream_info::AudioStreamInfo, video_stream_info::VideoStreamInfo},
    server::AudioPacket,
};

pub trait AirPlayConsumer: Send + Sync {
    fn on_video(&self, bytes: &[u8]);

    fn on_video_format(&self, video_stream_info: VideoStreamInfo);

    fn on_video_src_disconnect(&self);

    fn on_audio_format(&self, audio_stream_info: AudioStreamInfo);

    fn on_audio(&self, packet: &AudioPacket);

    fn on_audio_src_disconnect(&self);

    fn on_volume(&self, volume: f32);
}

pub type ArcAirPlayConsumer = Arc<dyn AirPlayConsumer>;
