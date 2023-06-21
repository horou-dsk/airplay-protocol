#![allow(clippy::uninit_vec)]

use std::io::Cursor;

use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::task::JoinHandle;
use tokio::{io, net::TcpListener};

use crate::airplay::airplay_consumer::ArcAirPlayConsumer;
use crate::airplay::lib::fairplay_video_decryptor::FairPlayVideoDecryptor;

#[derive(Default)]
pub struct VideoServer {
    server: Option<VideoServer1>,
}

impl VideoServer {
    pub async fn start(
        &mut self,
        video_decryptor: FairPlayVideoDecryptor,
        consumer: ArcAirPlayConsumer,
    ) -> io::Result<()> {
        self.server = Some(VideoServer1::start(video_decryptor, consumer).await?);
        Ok(())
    }

    pub fn get_port(&self) -> u16 {
        self.server.as_ref().unwrap().port()
    }

    pub fn stop(&mut self) {
        self.server.take();
    }
}

struct VideoServer1 {
    task: JoinHandle<()>,
    port: u16,
}

impl VideoServer1 {
    pub async fn start(
        video_decryptor: FairPlayVideoDecryptor,
        consumer: ArcAirPlayConsumer,
    ) -> io::Result<Self> {
        let listener = TcpListener::bind("0.0.0.0:0").await?;
        let port = listener.local_addr()?.port();
        let task = tokio::task::spawn(async move {
            log::info!("VideoServer Starting...");
            loop {
                let (stream, _) = listener.accept().await.unwrap();
                tokio::task::spawn(video_hanlde(
                    stream,
                    video_decryptor.clone(),
                    consumer.clone(),
                ));
            }
        });
        Ok(Self { task, port })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    // pub fn stop(self) {}
}

impl Drop for VideoServer1 {
    fn drop(&mut self) {
        self.task.abort();
    }
}

pub struct VideoPacket {
    pub payload_type: u16,
    pub payload_size: usize,
    pub payload: Vec<u8>,
}

enum DecoderState {
    ReadHeader,
    ReadPayload,
}

struct VideoDecoder {
    state: DecoderState,
    payload_size: usize,
    payload_type: u16,
    header_buf: [u8; 128],
    payload_buf: [u8; 2048],
}

impl VideoDecoder {
    fn new() -> Self {
        Self {
            state: DecoderState::ReadHeader,
            payload_size: 0,
            payload_type: 0,
            header_buf: [0; 128],
            payload_buf: [0; 2048],
        }
    }

    async fn decode(
        &mut self,
        reader: &mut BufReader<TcpStream>,
    ) -> io::Result<Option<VideoPacket>> {
        match self.state {
            DecoderState::ReadHeader => {
                reader.read_exact(&mut self.header_buf).await?;
                log::info!("header {:?}", self.header_buf);
                let mut head_cur = Cursor::new(&mut self.header_buf);
                self.payload_size = head_cur.read_u32_le().await? as usize;
                self.payload_type = head_cur.read_u16_le().await? & 0xFF;
                self.state = DecoderState::ReadPayload;
            }
            DecoderState::ReadPayload => {
                if self.payload_type == 0 || self.payload_type == 1 {
                    let mut payload_buf = Vec::with_capacity(self.payload_size);
                    unsafe { payload_buf.set_len(self.payload_size) };
                    reader.read_exact(&mut payload_buf).await?;
                    self.state = DecoderState::ReadHeader;
                    return Ok(Some(VideoPacket {
                        payload_type: self.payload_type,
                        payload_size: self.payload_size,
                        payload: payload_buf,
                    }));
                } else {
                    log::info!(
                        "Video packet with type: {}, length: {} bytes is skipped",
                        self.payload_type,
                        self.payload_size
                    );
                    let mut already_size = 0;
                    loop {
                        let len = (self.payload_size - already_size).min(self.payload_buf.len());
                        if len == 0 {
                            break;
                        }
                        reader.read_exact(&mut self.payload_buf[..len]).await?;
                        already_size += len;
                    }
                    self.state = DecoderState::ReadHeader;
                }
            }
        }
        Ok(None)
    }
}

fn prepare_picture_nal_units(payload: &mut [u8]) {
    let mut idx = 0;
    while idx < payload.len() {
        let nalu_size = i32::from_be_bytes(payload[idx..idx + 4].try_into().unwrap()) as usize; //payload[idx + 3] | (payload[idx + 2] << 8) | ((payload[idx + 1] & 0xFF) << 16) | ((payload[idx] & 0xFF) << 24);
        if nalu_size == 1 {
            break;
        }
        if nalu_size > 0 {
            payload[idx] = 0;
            payload[idx + 1] = 0;
            payload[idx + 2] = 0;
            payload[idx + 3] = 1;
            idx += nalu_size + 4;
        }
        if payload.len() - nalu_size > 4 {
            log::error!("Video packet contains corrupted NAL unit. It might be decrypt error");
            break;
        }
    }
}

fn prepare_sps_pps_nal_units(payload: &[u8]) -> Vec<u8> {
    let sps_size = u16::from_be_bytes(payload[6..8].try_into().unwrap()) as usize;
    let seq_par_set = &payload[9..9 + sps_size];

    let payload = &payload[9 + sps_size..];

    let pps_size = u16::from_be_bytes(payload[..2].try_into().unwrap()) as usize;
    let pps = &payload[3..3 + pps_size];

    let sps_pps_size = sps_size + pps_size + 8;
    log::info!("SPS PPS length: {}", sps_pps_size);

    let mut sps_pps = Vec::with_capacity(sps_pps_size);
    sps_pps.extend_from_slice(&[0, 0, 0, 1]);
    sps_pps.extend_from_slice(seq_par_set);
    sps_pps.extend_from_slice(&[0, 0, 0, 1]);
    sps_pps.extend_from_slice(pps);

    sps_pps
}

async fn video_hanlde(
    stream: TcpStream,
    mut video_decryptor: FairPlayVideoDecryptor,
    consumer: ArcAirPlayConsumer,
) {
    log::info!("VideoServer 连接进入...");
    let mut decoder = VideoDecoder::new();
    let mut reader = BufReader::new(stream);
    loop {
        log::info!("读取中...");
        let result = decoder.decode(&mut reader).await;
        match result {
            Ok(packet) => {
                if let Some(mut video_packet) = packet {
                    // log::info!(
                    //     "payload_type = {}, payload_size = {}",
                    //     video_packet.payload_type,
                    //     video_packet.payload_size
                    // );
                    match video_packet.payload_type {
                        0 => {
                            video_decryptor.decrypt(&mut video_packet.payload);
                            prepare_picture_nal_units(&mut video_packet.payload);
                            consumer.on_video(video_packet.payload.to_vec());
                        }
                        1 => {
                            consumer.on_video(prepare_sps_pps_nal_units(&video_packet.payload));
                        }
                        _ => (),
                    }
                }
            }
            Err(err) => {
                log::error!("video server read error! {:?}", err);
                break;
            }
        }
    }
    log::info!("VideoServer 连接断开...");
}
