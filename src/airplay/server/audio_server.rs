use std::collections::HashMap;

use tokio::{
    io::{self, AsyncBufReadExt, AsyncReadExt, BufReader},
    net::UdpSocket,
    sync::Mutex,
    task::JoinHandle,
};

use crate::airplay::{
    airplay_consumer::ArcAirPlayConsumer, lib::fairplay_audio_decryptor::FairPlayAudioDecryptor,
};

#[derive(Default)]
pub struct AudioServer {
    server: Mutex<Option<ServerInner>>,
}

impl AudioServer {
    pub async fn start(
        &self,
        audio_decryptor: FairPlayAudioDecryptor,
        consumer: ArcAirPlayConsumer,
    ) -> io::Result<()> {
        *self.server.lock().await = Some(ServerInner::start(audio_decryptor, consumer).await?);
        Ok(())
    }

    pub async fn get_port(&self) -> u16 {
        self.server.lock().await.as_ref().unwrap().port()
    }

    pub async fn stop(&self) {
        self.server.lock().await.take();
    }
}

struct ServerInner {
    task: JoinHandle<()>,
    port: u16,
}

impl ServerInner {
    async fn start(
        audio_decryptor: FairPlayAudioDecryptor,
        consumer: ArcAirPlayConsumer,
    ) -> io::Result<Self> {
        // TODO: 音频使用的是UDP并非TCP
        let listener = UdpSocket::bind("0.0.0.0:0").await?;
        let port = listener.local_addr()?.port();
        let task = tokio::task::spawn(async move {
            log::info!("AudioServer Starting... port = {}", port);
            audio_hanlde(listener, audio_decryptor.clone(), consumer.clone()).await
        });
        Ok(Self { task, port })
    }

    fn port(&self) -> u16 {
        self.port
    }
}

impl Drop for ServerInner {
    fn drop(&mut self) {
        log::info!("终止 Audio Server...");
        self.task.abort();
    }
}

#[allow(dead_code)]
struct AudioPacket {
    flag: u8,
    ty: u8,
    seq_number: u16,
    timestamp: u32,
    ssrc: u32,
    audio_buf: [u8; 480 * 4],
    audio_size: usize,
}

impl AudioPacket {
    fn audio_buf_mut(&mut self) -> &mut [u8] {
        &mut self.audio_buf[..self.audio_size]
    }

    fn audio_buf(&self) -> &[u8] {
        &self.audio_buf[..self.audio_size]
    }
}

struct AudioDecoder {
    header_buf: [u8; 12],
}

impl AudioDecoder {
    fn new() -> Self {
        Self {
            header_buf: [0; 12],
        }
    }

    async fn decode(&mut self, reader: &mut BufReader<&[u8]>) -> io::Result<AudioPacket> {
        reader.read_exact(&mut self.header_buf).await?;
        let flag = self.header_buf[0];
        let ty = self.header_buf[1];
        let seq_number = u16::from_be_bytes(self.header_buf[2..4].try_into().unwrap());

        let timestamp = u32::from_be_bytes(self.header_buf[4..8].try_into().unwrap()); //(headerBytes[7] & 0xFF) | ((headerBytes[6] & 0xFF) << 8) |
                                                                                       // ((headerBytes[5] & 0xFF) << 16) | ((headerBytes[4] & 0xFF) << 24);
        let ssrc = u32::from_be_bytes(self.header_buf[8..].try_into().unwrap());
        // long ssrc = (headerBytes[11] & 0xFF) | ((headerBytes[6] & 0xFF) << 8) |
        // ((headerBytes[9] & 0xFF) << 16) | ((headerBytes[8] & 0xFF) << 24);
        let buf_size = reader.fill_buf().await?.len();
        let mut audio_buf = [0; 480 * 4];
        let audio_size = buf_size.min(audio_buf.len());
        reader.read_exact(&mut audio_buf[..audio_size]).await?;

        let audio_packet = AudioPacket {
            flag,
            ty,
            seq_number,
            timestamp,
            ssrc,
            audio_buf,
            audio_size,
        };
        Ok(audio_packet)
    }
}

async fn audio_hanlde(
    listener: UdpSocket,
    audio_decryptor: FairPlayAudioDecryptor,
    consumer: ArcAirPlayConsumer,
) {
    log::info!("AudioServer 启动...");
    let mut decoder = AudioDecoder::new();
    let mut prev_seq_num = 0;
    let mut buf = [0; 4096];
    let mut packet_buf = HashMap::new();
    let packet_buf_len = 512;
    loop {
        let (read_bytes, _from) = listener.recv_from(&mut buf).await.unwrap();
        // log::info!("读取到音频数据 大小 = {read_bytes}...");
        // let now = Instant::now();
        let mut reader = BufReader::new(&buf[..read_bytes]);
        let result = decoder.decode(&mut reader).await;
        match result {
            Ok(packet) => {
                let mut cur_seq_num = packet.seq_number;
                if cur_seq_num < prev_seq_num {
                    continue;
                }
                let key = cur_seq_num % packet_buf_len;
                packet_buf.insert(key, packet);
                while 'd: {
                    if cur_seq_num - prev_seq_num == 1 || prev_seq_num == 0 {
                        if let Some(mut packet) = packet_buf.remove(&key) {
                            audio_decryptor.decrypt(packet.audio_buf_mut());
                            consumer.on_audio(packet.audio_buf().to_vec());
                            prev_seq_num = cur_seq_num;
                            break 'd true;
                        }
                    }
                    false
                } {
                    cur_seq_num += 1;
                }
                // log::info!("耗时 {:?}", now.elapsed());
            }
            Err(err) => {
                log::error!("video server read error! {:?}", err);
                break;
            }
        }
    }
    log::info!("AudioServer 结束...");
}
