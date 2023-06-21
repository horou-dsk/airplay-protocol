use tokio::{
    io::{self, AsyncBufReadExt, AsyncReadExt, BufReader},
    net::{TcpListener, TcpStream},
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

    pub async fn stop(&mut self) {
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
        let listener = TcpListener::bind("0.0.0.0:0").await?;
        let port = listener.local_addr()?.port();
        let task = tokio::task::spawn(async move {
            log::info!("AudioServer Starting...");
            loop {
                let (stream, _) = listener.accept().await.unwrap();
                tokio::task::spawn(audio_hanlde(
                    stream,
                    audio_decryptor.clone(),
                    consumer.clone(),
                ));
            }
        });
        Ok(Self { task, port })
    }

    fn port(&self) -> u16 {
        self.port
    }
}

impl Drop for ServerInner {
    fn drop(&mut self) {
        self.task.abort();
    }
}

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

    async fn decode(&mut self, reader: &mut BufReader<TcpStream>) -> io::Result<AudioPacket> {
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
    stream: TcpStream,
    audio_decryptor: FairPlayAudioDecryptor,
    consumer: ArcAirPlayConsumer,
) {
    log::info!("AudioServer 连接进入...");
    let mut decoder = AudioDecoder::new();
    let mut reader = BufReader::new(stream);
    let prev_seq_num = 0;
    loop {
        log::info!("读取中...");
        let result = decoder.decode(&mut reader).await;
        match result {
            Ok(mut packet) => {
                let cur_seq_num = packet.seq_number;
                if cur_seq_num < prev_seq_num {
                    continue;
                }
                audio_decryptor.decrypt(packet.audio_buf_mut());
                consumer.on_audio(packet.audio_buf().to_vec());
            }
            Err(err) => {
                log::error!("video server read error! {:?}", err);
                break;
            }
        }
    }
    log::info!("AudioServer 连接断开...");
}
