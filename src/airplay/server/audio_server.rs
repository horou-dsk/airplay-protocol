use tokio::{io, net::UdpSocket, sync::Mutex, task::JoinHandle};

use crate::airplay::{
    airplay_consumer::ArcAirPlayConsumer, lib::fairplay_audio_decryptor::FairPlayAudioDecryptor,
};

use super::AudioPacket;

#[derive(Default)]
pub struct AudioServer {
    server: Mutex<Option<ServerInner>>,
}

impl AudioServer {
    pub async fn start(
        &self,
        audio_decryptor: FairPlayAudioDecryptor,
        consumer: ArcAirPlayConsumer,
        audio_buffer_size: Option<u16>,
    ) -> io::Result<()> {
        *self.server.lock().await =
            Some(ServerInner::start(audio_decryptor, consumer, audio_buffer_size).await?);
        Ok(())
    }

    pub async fn get_port(&self) -> u16 {
        self.server.lock().await.as_ref().unwrap().port()
    }

    pub async fn stop(&self) {
        self.server.lock().await.take();
    }

    pub async fn is_running(&self) -> bool {
        self.server.lock().await.is_some()
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
        audio_buffer_size: Option<u16>,
    ) -> io::Result<Self> {
        let listener = UdpSocket::bind("0.0.0.0:0").await?;
        let port = listener.local_addr()?.port();
        let task = tokio::task::spawn(async move {
            log::info!("AudioServer Starting... port = {}", port);
            audio_hanlde(listener, audio_decryptor, consumer, audio_buffer_size).await
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

struct AudioDecoder(AudioPacket);

impl AudioDecoder {
    async fn decode(&mut self, reader: &[u8]) -> io::Result<&AudioPacket> {
        let header_buf = &reader[..12];
        let body_buf = &reader[12..];
        let flag = header_buf[0];
        let ty = header_buf[1] & 0x7F;

        let seq_number = u16::from_be_bytes(header_buf[2..4].try_into().unwrap());

        let timestamp = u32::from_be_bytes(header_buf[4..8].try_into().unwrap());

        let ssrc = u32::from_be_bytes(header_buf[8..].try_into().unwrap());
        // let mut audio_buf = [0; 480 * 4];
        // TODO: may be out of bounds
        // audio_buf[..body_buf.len()].copy_from_slice(body_buf);

        self.0.flag = flag;
        self.0.ty = ty;
        self.0.seq_number = seq_number;
        self.0.timestamp = timestamp;
        self.0.ssrc = ssrc;
        self.0.audio_buf[..body_buf.len()].copy_from_slice(body_buf);
        self.0.audio_size = body_buf.len();
        self.0.filled = true;
        // let audio_packet = AudioPacket {
        //     flag,
        //     ty,
        //     seq_number,
        //     timestamp,
        //     ssrc,
        //     audio_buf,
        //     audio_size: body_buf.len(),
        // };
        Ok(&self.0)
    }
}

const MAX_AUDIO_BUFFER_SIZE: usize = 32;
const DEFAULT_AUDIO_BUFFER_SIZE: u16 = 32;

#[inline]
fn seqnum_cmp(s1: u16, s2: u16) -> i16 {
    (s1 - s2) as i16
}

struct AudioBuffer {
    first_seqnum: u16,
    last_seqnum: u16,
    is_empty: bool,
    buffer_size: u16,
    entries: [AudioPacket; MAX_AUDIO_BUFFER_SIZE],
}

impl Default for AudioBuffer {
    fn default() -> Self {
        Self {
            is_empty: true,
            last_seqnum: 0,
            first_seqnum: 0,
            buffer_size: DEFAULT_AUDIO_BUFFER_SIZE,
            entries: Default::default(),
        }
    }
}

impl AudioBuffer {
    fn with_buffer_size(size: u16) -> Self {
        Self {
            buffer_size: size.min(MAX_AUDIO_BUFFER_SIZE as u16),
            ..Default::default()
        }
    }
}

impl AudioBuffer {
    fn buffer_flush(&mut self, next_seq: u16) {
        self.entries
            .iter_mut()
            .for_each(|entry| entry.filled = false);
        if !(0..=0xFFFF).contains(&next_seq) {
            self.is_empty = true;
        } else {
            self.first_seqnum = next_seq;
            self.last_seqnum = next_seq - 1;
        }
        log::info!("Audio Buffer Flush");
    }

    fn buffer_enqueue(&mut self, packet: &AudioPacket) {
        let seq_number = packet.seq_number;

        // If this packet is too late, just skip it
        if !self.is_empty && seqnum_cmp(seq_number, self.first_seqnum) < 0 {
            return;
        }

        if seqnum_cmp(seq_number, self.first_seqnum + self.buffer_size) >= 0 {
            self.buffer_flush(seq_number);
        }

        let entry = &mut self.entries[(seq_number % self.buffer_size) as usize];
        if entry.filled && seqnum_cmp(entry.seq_number, seq_number) == 0 {
            return;
        }

        entry.copy(packet);

        if self.is_empty {
            self.first_seqnum = seq_number;
            self.last_seqnum = seq_number;
            self.is_empty = false;
        }
        if seqnum_cmp(seq_number, self.last_seqnum) > 0 {
            self.last_seqnum = seq_number;
        }
    }

    fn buffer_dequeue(&mut self) -> Option<&mut AudioPacket> {
        let entry_count = seqnum_cmp(self.last_seqnum, self.first_seqnum) + 1;
        if self.is_empty || entry_count <= 0 {
            return None;
        }

        let entry = &mut self.entries[(self.first_seqnum % self.buffer_size) as usize];
        if !entry.filled {
            /* Check how much we have space left in the buffer */
            if entry_count < self.buffer_size as i16 {
                /* Return nothing and hope resend gets on time */
                return None;
            }
            /* Risk of buffer overrun, return empty buffer */
        }
        self.first_seqnum += 1;
        if entry.filled {
            entry.filled = false;
            Some(entry)
        } else {
            None
        }
    }
}

async fn audio_hanlde(
    listener: UdpSocket,
    audio_decryptor: FairPlayAudioDecryptor,
    consumer: ArcAirPlayConsumer,
    audio_buffer_size: Option<u16>,
) {
    log::info!("AudioServer new connection coming in...");
    let mut buf = [0; 4096];
    let mut audio_buffer =
        AudioBuffer::with_buffer_size(audio_buffer_size.unwrap_or(MAX_AUDIO_BUFFER_SIZE as u16));
    let mut decoder = AudioDecoder(Default::default());
    while let Ok(read_bytes) = listener.recv(&mut buf).await {
        let buf = &buf[..read_bytes];
        if read_bytes == 16 && buf[12] == 0x0 && buf[13] == 0x68 && buf[14] == 0x34 && buf[15] == 0
        {
            continue;
        }
        let result = decoder.decode(buf).await;
        match result {
            Ok(packet) => {
                log::debug!(
                    "cur_seq_num = {}, first_seq_num = {}, last_seq_num = {}",
                    packet.seq_number,
                    audio_buffer.first_seqnum,
                    audio_buffer.last_seqnum
                );
                audio_buffer.buffer_enqueue(packet);
                while let Some(packet) = audio_buffer.buffer_dequeue() {
                    audio_decryptor.decrypt(packet.audio_buf_mut());
                    consumer.on_audio(packet);
                }
            }
            Err(err) => {
                log::error!("audio server read error! {:?}", err);
                break;
            }
        }
    }
    log::info!("AudioServer disconnected...");
}
