use std::io::Write;
use std::net::SocketAddr;
use std::sync::Arc;

use airplay2_protocol::airplay::airplay_consumer::{AirPlayConsumer, ArcAirPlayConsumer};
use airplay2_protocol::airplay::AirPlayConfig;
use airplay2_protocol::airplay_bonjour::AirPlayBonjour;
use airplay2_protocol::control_handle::ControlHandle;
use airplay2_protocol::net::server::Server as MServer;
// use env_logger::Env;

struct VideoConsumer;

impl AirPlayConsumer for VideoConsumer {
    fn on_video(&self, bytes: Vec<u8>) {
        log::info!("on_video...");
    }

    fn on_video_format(
        &self,
        video_stream_info: airplay2_protocol::airplay::lib::video_stream_info::VideoStreamInfo,
    ) {
        log::info!("on_video format...");
    }

    fn on_video_src_disconnect(&self) {
        log::info!("on_video disconnect...");
    }

    fn on_audio_format(
        &self,
        audio_stream_info: airplay2_protocol::airplay::lib::audio_stream_info::AudioStreamInfo,
    ) {
        log::info!("on_audio_format...");
    }

    fn on_audio(&self, bytes: Vec<u8>) {
        log::info!("on_audio...");
    }
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut builder = env_logger::Builder::from_default_env();
    // let env = Env::default().filter_or("MY_LOG_LEVEL", "info");
    // builder.parse_filters("libmdns=debug");
    // builder.filter_level(env);
    builder.format(|buf, record| {
        let mut style = buf.style();
        style.set_bold(true);
        match record.level() {
            log::Level::Error => {style.set_color(env_logger::fmt::Color::Red);},
            log::Level::Warn => {style.set_color(env_logger::fmt::Color::Yellow);},
            log::Level::Info => {style.set_color(env_logger::fmt::Color::Green);},
            _ => ()
            // log::Level::Debug => todo!(),
            // log::Level::Trace => todo!(),
        };
        writeln!(
            buf,
            "[{} {} {}:{}] {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            style.value(record.level()),
            record.file().unwrap_or("unknown"),
            record.line().unwrap_or(0),
            record.args()
        )
    });
    builder.init();

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
        port,
    };
    let video_consumer: ArcAirPlayConsumer = Arc::new(Box::new(VideoConsumer));
    let mserver = MServer::bind(
        addr,
        ControlHandle::new(airplay_config, video_consumer.clone(), video_consumer),
    );
    mserver.run().await?;
    Ok(())
}
