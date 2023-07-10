use std::net::SocketAddr;
use std::sync::Arc;

use airplay2_protocol::airplay::airplay_consumer::{AirPlayConsumer, ArcAirPlayConsumer};
use airplay2_protocol::airplay::AirPlayConfig;
use airplay2_protocol::airplay_bonjour::AirPlayBonjour;
use airplay2_protocol::control_handle::ControlHandle;
use airplay2_protocol::net::server::Server as MServer;
use airplay2_protocol::setup_log;
// use env_logger::Env;

struct VideoConsumer;

impl AirPlayConsumer for VideoConsumer {
    fn on_video(&self, _bytes: Vec<u8>) {
        log::info!("on_video...");
    }

    fn on_video_format(
        &self,
        _video_stream_info: airplay2_protocol::airplay::lib::video_stream_info::VideoStreamInfo,
    ) {
        log::info!("on_video format...");
    }

    fn on_video_src_disconnect(&self) {
        log::info!("on_video disconnect...");
    }

    fn on_audio_format(
        &self,
        _audio_stream_info: airplay2_protocol::airplay::lib::audio_stream_info::AudioStreamInfo,
    ) {
        log::info!("on_audio_format...");
    }

    fn on_audio(&self, _bytes: Vec<u8>) {
        log::info!("on_audio...");
    }

    fn on_audio_src_disconnect(&self) {
        log::info!("on audio disconnect");
    }

    fn on_volume(&self, volume: f32) {
        log::info!("volume = {volume}");
    }
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    setup_log();
    let port = 31927;
    let name = "RustAirplay";

    // pin码认证功能缺失...
    let _air = AirPlayBonjour::new(name, port, false);

    let addr: SocketAddr = ([0, 0, 0, 0], port).into();
    let airplay_config = AirPlayConfig {
        server_name: name.to_string(),
        width: 1920,
        height: 1080,
        fps: 30,
    };
    let video_consumer: ArcAirPlayConsumer = Arc::new(VideoConsumer);
    let mserver = MServer::bind_with_addr(
        addr,
        ControlHandle::new(airplay_config, video_consumer.clone(), video_consumer),
    )
    .await;
    mserver.run().await?;
    Ok(())
}
