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
        let listener = UdpSocket::bind("0.0.0.0:0").await?;
        let port = listener.local_addr()?.port();
        let task = tokio::task::spawn(async move {
            log::info!("AudioServer Starting... port = {}", port);
            audio_hanlde(listener, audio_decryptor, consumer).await
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
    async fn decode(reader: &[u8]) -> io::Result<AudioPacket> {
        let header_buf = &reader[..12];
        let body_buf = &reader[12..];
        let flag = header_buf[0];
        let ty = header_buf[1] & 0x7F;

        let seq_number = u16::from_be_bytes(header_buf[2..4].try_into().unwrap());

        let timestamp = u32::from_be_bytes(header_buf[4..8].try_into().unwrap());

        let ssrc = u32::from_be_bytes(header_buf[8..].try_into().unwrap());
        let mut audio_buf = [0; 480 * 4];
        // TODO: may be out of bounds
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

const AUDIO_BUFFER_LEN: u16 = 32;

fn seqnum_cmp(s1: u16, s2: u16) -> i16 {
    (s1 - s2) as i16
}

struct AudioBuffer {
    first_seqnum: u16,
    last_seqnum: u16,
    is_empty: bool,
    entries: [Option<AudioPacket>; AUDIO_BUFFER_LEN as usize],
}

impl Default for AudioBuffer {
    fn default() -> Self {
        Self {
            is_empty: true,
            last_seqnum: 0,
            first_seqnum: 0,
            entries: Default::default(),
        }
    }
}

impl AudioBuffer {
    fn buffer_flush(&mut self, next_seq: u16) {
        self.entries.iter_mut().for_each(|entry| *entry = None);
        if !(0..=0xFFFF).contains(&next_seq) {
            self.is_empty = true;
        } else {
            self.first_seqnum = next_seq;
            self.last_seqnum = next_seq - 1;
        }
    }

    fn buffer_enqueue(&mut self, packet: AudioPacket) {
        let seq_number = packet.seq_number;

        // If this packet is too late, just skip it
        if !self.is_empty && seqnum_cmp(seq_number, self.first_seqnum) < 0 {
            return;
        }

        if seqnum_cmp(seq_number, self.first_seqnum + AUDIO_BUFFER_LEN) >= 0 {
            self.buffer_flush(seq_number);
        }

        let entry = &mut self.entries[(seq_number % AUDIO_BUFFER_LEN) as usize];
        if let Some(entry) = entry {
            if seqnum_cmp(entry.seq_number, seq_number) == 0 {
                return;
            }
        }

        *entry = Some(packet);

        if self.is_empty {
            self.first_seqnum = seq_number;
            self.last_seqnum = seq_number;
            self.is_empty = false;
        }
        if seqnum_cmp(seq_number, self.last_seqnum) > 0 {
            self.last_seqnum = seq_number;
        }
    }

    fn buffer_dequeue(&mut self) -> Option<AudioPacket> {
        let entry_count: i16 = (self.last_seqnum - self.first_seqnum) as i16 + 1;
        if self.is_empty || entry_count <= 0 {
            return None;
        }

        let entry = self.entries[(self.first_seqnum % AUDIO_BUFFER_LEN) as usize].take();
        if entry.is_none() && entry_count < AUDIO_BUFFER_LEN as i16 {
            None
        } else {
            self.first_seqnum += 1;
            entry
        }
    }
}

async fn audio_hanlde(
    listener: UdpSocket,
    audio_decryptor: FairPlayAudioDecryptor,
    consumer: ArcAirPlayConsumer,
) {
    log::info!("AudioServer 启动...");
    let mut buf = [0; 4096];
    let mut audio_buffer = AudioBuffer::default();
    loop {
        let read_bytes = listener.recv(&mut buf).await.unwrap();

        let buf = &buf[..read_bytes];
        if read_bytes == 16 && buf[12] == 0x0 && buf[13] == 0x68 && buf[14] == 0x34 && buf[15] == 0
        {
            continue;
        }
        // log::info!("读取到音频数据 大小 = {read_bytes}...");
        // let now = Instant::now();
        let result = AudioDecoder::decode(buf).await;
        match result {
            Ok(packet) => {
                log::debug!(
                    "cur_seq_num = {}, first_seq_num = {}, last_seq_num = {}",
                    packet.seq_number,
                    audio_buffer.first_seqnum,
                    audio_buffer.last_seqnum
                );
                audio_buffer.buffer_enqueue(packet);
                while let Some(mut packet) = audio_buffer.buffer_dequeue() {
                    audio_decryptor.decrypt(packet.audio_buf_mut());
                    consumer.on_audio(packet.audio_buf().to_vec());
                }
                // log::info!("耗时 {:?}", now.elapsed());
            }
            Err(err) => {
                log::error!("audio server read error! {:?}", err);
                break;
            }
        }
    }
    log::info!("AudioServer 结束...");
}
