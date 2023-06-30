use std::io::{BufReader, Write};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use airplay2_protocol::airplay::airplay_consumer::{AirPlayConsumer, ArcAirPlayConsumer};
use airplay2_protocol::airplay::AirPlayConfig;
use airplay2_protocol::airplay_bonjour::AirPlayBonjour;
use airplay2_protocol::control_handle::ControlHandle;
use airplay2_protocol::net::server::Server as MServer;
use crossbeam::channel::{Receiver, Sender};
use rodio::{Decoder, OutputStream, Source};

struct MediaBuffer {
    rx: Receiver<Vec<u8>>,
    v: Vec<u8>,
}

impl std::io::Read for MediaBuffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.v.len();
        if len > 0 {
            let len = len.min(buf.len());
            let result: Vec<u8> = self.v.drain(..len).collect();
            buf[..len].copy_from_slice(&result);
            Ok(len)
        } else {
            match self.rx.recv() {
                Ok(mut v) => {
                    let len = v.len().min(buf.len());
                    let result: Vec<u8> = v.drain(..len).collect();
                    buf[..len].copy_from_slice(&result);
                    if !v.is_empty() {
                        self.v.extend(v);
                    }
                    Ok(len)
                }
                Err(err) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err)),
            }
        }
    }
}

impl std::io::Seek for MediaBuffer {
    fn seek(&mut self, _pos: std::io::SeekFrom) -> std::io::Result<u64> {
        Ok(0)
    }
}

struct MediaPlay {
    tx: Sender<Vec<u8>>,
}

impl MediaPlay {
    pub fn new() -> Self {
        let (tx, rx) = crossbeam::channel::unbounded();
        std::thread::spawn(move || {
            let audio_stream = OutputStream::try_default().unwrap();
            let audio_buffer = MediaBuffer { rx, v: Vec::new() };
            let reader = BufReader::new(audio_buffer);
            // TODO：需要使用 alac 解码
            let source = Decoder::new(reader).unwrap();
            audio_stream.1.play_raw(source.convert_samples()).unwrap();
            std::thread::sleep(Duration::from_secs(999));
        });

        Self { tx }
    }
}

impl MediaPlay {
    pub fn put_buf(&self, buf: Vec<u8>) {
        self.tx.send(buf).unwrap();
    }
}

struct VideoConsumer {
    media_play: MediaPlay,
}

impl VideoConsumer {
    pub fn new() -> Self {
        Self {
            media_play: MediaPlay::new(),
        }
    }
}

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
        audio_stream_info: airplay2_protocol::airplay::lib::audio_stream_info::AudioStreamInfo,
    ) {
        log::info!(
            "on_audio_format...type = {:?}",
            audio_stream_info.compression_type
        );
    }

    fn on_audio(&self, bytes: Vec<u8>) {
        self.media_play.put_buf(bytes);
        log::info!("on_audio...");
    }

    fn on_audio_src_disconnect(&self) {}

    fn on_volume(&self, _volume: f32) {}
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut builder = env_logger::Builder::from_default_env();
    builder.format(|buf, record| {
        let mut style = buf.style();
        style.set_bold(true);
        match record.level() {
            log::Level::Error => {
                style.set_color(env_logger::fmt::Color::Red);
            }
            log::Level::Warn => {
                style.set_color(env_logger::fmt::Color::Yellow);
            }
            log::Level::Info => {
                style.set_color(env_logger::fmt::Color::Green);
            }
            _ => (),
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

    let _air = AirPlayBonjour::new(name, port, false);

    let addr: SocketAddr = ([0, 0, 0, 0], port).into();
    let airplay_config = AirPlayConfig {
        server_name: name.to_string(),
        width: 1920,
        height: 1080,
        fps: 30,
        port,
    };
    let video_consumer: ArcAirPlayConsumer = Arc::new(Box::new(VideoConsumer::new()));
    let mserver = MServer::bind(
        addr,
        ControlHandle::new(airplay_config, video_consumer.clone(), video_consumer),
    );
    mserver.run().await?;
    Ok(())
}
