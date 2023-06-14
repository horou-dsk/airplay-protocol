use std::time::Duration;

use bytes::{Buf, Bytes, BytesMut};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::task::JoinHandle;
use tokio::{io, net::TcpListener};

#[derive(Default)]
pub struct VideoServer {
    server: Option<VideoServer1>,
}

impl VideoServer {
    pub async fn start(&mut self) -> io::Result<()> {
        self.server = Some(VideoServer1::start().await?);
        Ok(())
    }

    pub fn get_port(&self) -> u16 {
        self.server.as_ref().unwrap().port()
    }
}

struct VideoServer1 {
    task: JoinHandle<()>,
    port: u16,
}

impl VideoServer1 {
    pub async fn start() -> io::Result<Self> {
        let listener = TcpListener::bind("0.0.0.0:0").await?;
        let port = listener.local_addr()?.port();
        let task = tokio::task::spawn(async move {
            log::info!("VideoServer Starting...");
            loop {
                let (stream, _) = listener.accept().await.unwrap();
                tokio::task::spawn(video_hanlde(stream));
            }
        });
        Ok(Self { task, port })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn stop(self) {}
}

impl Drop for VideoServer1 {
    fn drop(&mut self) {
        self.task.abort();
    }
}

pub struct VideoPacket {
    pub payload_type: u8,
    pub payload_size: usize,
    pub payload: Bytes,
}

enum DecoderState {
    ReadHeader,
    ReadPayload,
}

struct VideoDecoder {
    state: DecoderState,
    payload_size: u32,
    payload_type: u8,
}

impl VideoDecoder {
    fn new() -> Self {
        Self {
            state: DecoderState::ReadHeader,
            payload_size: 0,
            payload_type: 0,
        }
    }

    async fn decode(
        &mut self,
        reader: &mut BufReader<&mut TcpStream>,
    ) -> io::Result<Option<VideoPacket>> {
        match self.state {
            DecoderState::ReadHeader => {
                let mut header_buf = [0; 128];
                reader.read_exact(&mut header_buf).await?;
                let mut header_buf = &header_buf[..];
                self.payload_size = header_buf.get_u32_le();
                self.payload_type = header_buf.get_u8();
                self.state = DecoderState::ReadPayload;
            }
            DecoderState::ReadPayload => {
                if self.payload_type == 1 || self.payload_type == 2 {
                    let mut payload_buf = BytesMut::with_capacity(self.payload_size as usize);
                    reader.read_exact(&mut payload_buf).await?;
                    self.state = DecoderState::ReadHeader;
                    return Ok(Some(VideoPacket {
                        payload_type: self.payload_type,
                        payload_size: self.payload_size as usize,
                        payload: payload_buf.freeze(),
                    }));
                } else {
                    log::info!(
                        "Video packet with type: {}, length: {} bytes is skipped",
                        self.payload_type,
                        self.payload_size
                    );
                    reader.consume(self.payload_size as usize);
                    self.state = DecoderState::ReadHeader;
                }
            }
        }
        Ok(None)
    }
}

async fn video_hanlde(mut stream: TcpStream) {
    log::info!("VideoServer 连接进入...");
    let mut decoder = VideoDecoder::new();
    loop {
        let mut reader = BufReader::new(&mut stream);
        let result =
            tokio::time::timeout(Duration::from_secs(10), decoder.decode(&mut reader)).await;
        if let Ok(Ok(packet)) = result {
            if let Some(video_packet) = packet {
                log::info!(
                    "payload_type = {}, payload_size = {}",
                    video_packet.payload_type,
                    video_packet.payload_size
                );
            }
            // if amt == 0 {
            //     log::warn!("read none...................");
            //     // break;
            // }
        } else {
            break;
        }
    }
    log::info!("VideoServer 连接断开...");
}
