#[derive(Clone)]
pub struct AudioPacket {
    flag: u8,
    ty: u8,
    seq_number: u16,
    timestamp: u32,
    ssrc: u32,
    audio_buf: [u8; 480 * 4],
    audio_size: usize,
    filled: bool,
}

impl Default for AudioPacket {
    fn default() -> Self {
        Self {
            audio_buf: [0; 480 * 4],
            flag: 0,
            ty: 0,
            seq_number: 0,
            timestamp: 0,
            ssrc: 0,
            audio_size: 0,
            filled: false,
        }
    }
}

impl AudioPacket {
    fn audio_buf_mut(&mut self) -> &mut [u8] {
        &mut self.audio_buf[..self.audio_size]
    }

    fn copy(&mut self, packet: &AudioPacket) {
        self.flag = packet.flag;
        self.ty = packet.ty;
        self.seq_number = packet.seq_number;
        self.timestamp = packet.timestamp;
        self.ssrc = packet.ssrc;
        self.filled = packet.filled;
        self.audio_size = packet.audio_size;
        self.audio_buf[..packet.audio_size].copy_from_slice(&packet.audio_buf[..packet.audio_size]);
    }

    pub fn audio_buf(&self) -> &[u8] {
        &self.audio_buf[..self.audio_size]
    }

    pub fn timestamp(&self) -> u32 {
        self.timestamp
    }
}

pub mod audio_server;
pub mod video_server;
