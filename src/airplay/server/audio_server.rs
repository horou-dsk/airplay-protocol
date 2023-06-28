use tokio::{io, net::UdpSocket, sync::Mutex, task::JoinHandle};

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
#[derive(Clone)]
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

struct AudioDecoder;

impl AudioDecoder {
    // fn new() -> Self {
    //     Self {
    //         header_buf: [0; 12],
    //     }
    // }

    async fn decode(reader: &[u8]) -> io::Result<AudioPacket> {
        let header_buf = &reader[..12];
        let body_buf = &reader[12..];
        let flag = header_buf[0];
        let ty = header_buf[1];

        let seq_number = u16::from_be_bytes(header_buf[2..4].try_into().unwrap());

        let timestamp = u32::from_be_bytes(header_buf[4..8].try_into().unwrap());

        let ssrc = u32::from_be_bytes(header_buf[8..].try_into().unwrap());
        let mut audio_buf = [0; 480 * 4];
        audio_buf[..body_buf.len()].copy_from_slice(body_buf);

        let audio_packet = AudioPacket {
            flag,
            ty,
            seq_number,
            timestamp,
            ssrc,
            audio_buf,
            audio_size: body_buf.len(),
        };
        Ok(audio_packet)
    }
}

async fn audio_hanlde(
    listener: UdpSocket,
    audio_decryptor: FairPlayAudioDecryptor,
    consumer: ArcAirPlayConsumer,
) {
    log::warn!("AudioServer 启动...");
    let mut prev_seq_num = 0;
    let mut buf = [0; 4096];
    let packet_buf_len = 256;
    let mut packet_buf = vec![None; packet_buf_len];
    let mut packets_buf = 0;
    loop {
        let read_bytes = listener.recv(&mut buf).await.unwrap();
        // log::info!("读取到音频数据 大小 = {read_bytes}...");
        // let now = Instant::now();
        let result = AudioDecoder::decode(&buf[..read_bytes]).await;
        match result {
            Ok(packet) => {
                let mut cur_seq_num = packet.seq_number as usize;
                println!(
                    "cur_seq_num = {}, prev_seq_num = {}",
                    cur_seq_num, prev_seq_num
                );
                if cur_seq_num == 0 {
                    prev_seq_num = 0;
                }
                if cur_seq_num < prev_seq_num {
                    continue;
                }
                if packets_buf >= 3 {
                    prev_seq_num = cur_seq_num - 1;
                }
                let key = cur_seq_num % packet_buf_len;
                packet_buf[key] = Some(packet);
                while 'd: {
                    if cur_seq_num - prev_seq_num == 1 || prev_seq_num == 0 {
                        let packet = packet_buf[cur_seq_num % packet_buf_len].take();
                        if let Some(mut packet) = packet {
                            audio_decryptor.decrypt(packet.audio_buf_mut());
                            consumer.on_audio(packet.audio_buf().to_vec());
                            prev_seq_num = cur_seq_num;
                            packets_buf = 0;
                            break 'd true;
                        }
                    }
                    packets_buf += 1;
                    false
                } {
                    cur_seq_num += 1;
                }
                // log::info!("耗时 {:?}", now.elapsed());
            }
            Err(err) => {
                log::error!("audio server read error! {:?}", err);
                break;
            }
        }
    }
    log::warn!("AudioServer 结束...");
}
