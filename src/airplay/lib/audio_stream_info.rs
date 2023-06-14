#[derive(Debug)]
pub enum CompressionType {
    Lpcm,
    Alac,
    Aac,
    AacEld,
    Opus,
}

impl CompressionType {
    pub fn from_code(code: i64) -> Self {
        match code {
            1 => Self::Lpcm,
            2 => Self::Alac,
            4 => Self::Aac,
            8 => Self::AacEld,
            32 => Self::Opus,
            _ => panic!("error code {}", code),
        }
    }
}

#[derive(Debug)]
pub enum AudioFormat {
    Pcm8000_16_1,
    Pcm8000_16_2,
    Pcm16000_16_1,
    Pcm16000_16_2,
    Pcm24000_16_1,
    Pcm24000_16_2,
    Pcm32000_16_1,
    Pcm32000_16_2,
    Pcm44100_16_1,
    Pcm44100_16_2,
    Pcm44100_24_1,
    Pcm44100_24_2,
    Pcm48000_16_1,
    Pcm48000_16_2,
    Pcm48000_24_1,
    Pcm48000_24_2,
    Alac44100_16_2,
    Alac44100_24_2,
    Alac48000_16_2,
    Alac48000_24_2,
    AacLc44100_2,
    AacLc48000_2,
    AacEld44100_2,
    AacEld48000_2,
    AacEld16000_1,
    AacEld24000_1,
    Opus16000_1,
    Opus24000_1,
    Opus48000_1,
    AacEld44100_1,
    AacEld48000_1,
}

impl AudioFormat {
    pub fn from_code(code: i64) -> Self {
        match code {
            0x4 => Self::Pcm8000_16_1,
            0x8 => Self::Pcm8000_16_2,
            0x10 => Self::Pcm16000_16_1,
            0x20 => Self::Pcm16000_16_2,
            0x40 => Self::Pcm24000_16_1,
            0x80 => Self::Pcm24000_16_2,
            0x100 => Self::Pcm32000_16_1,
            0x200 => Self::Pcm32000_16_2,
            0x400 => Self::Pcm44100_16_1,
            0x800 => Self::Pcm44100_16_2,
            0x1000 => Self::Pcm44100_24_1,
            0x2000 => Self::Pcm44100_24_2,
            0x4000 => Self::Pcm48000_16_1,
            0x8000 => Self::Pcm48000_16_2,
            0x10000 => Self::Pcm48000_24_1,
            0x20000 => Self::Pcm48000_24_2,

            0x40000 => Self::Alac44100_16_2,
            0x80000 => Self::Alac44100_24_2,
            0x100000 => Self::Alac48000_16_2,
            0x200000 => Self::Alac48000_24_2,

            0x400000 => Self::AacLc44100_2,
            0x800000 => Self::AacLc48000_2,

            0x1000000 => Self::AacEld44100_2,
            0x2000000 => Self::AacEld48000_2,
            0x4000000 => Self::AacEld16000_1,
            0x8000000 => Self::AacEld24000_1,

            0x10000000 => Self::Opus16000_1,
            0x20000000 => Self::Opus24000_1,
            0x40000000 => Self::Opus48000_1,

            0x80000000 => Self::AacEld44100_1,
            0x100000000 => Self::AacEld48000_1,
            _ => panic!("unknow code {}", code),
        }
    }
}

#[derive(Debug)]
pub struct AudioStreamInfo {
    pub compression_type: CompressionType,
    pub audio_format: AudioFormat,
    pub samples_per_frame: u64,
}

impl AudioStreamInfo {
    pub(super) fn builder() -> AudioStreamInfoBuilder {
        AudioStreamInfoBuilder {
            audio_stream_info: Self {
                compression_type: CompressionType::Lpcm,
                audio_format: AudioFormat::Pcm44100_24_1,
                samples_per_frame: 0,
            },
        }
    }
}

pub(super) struct AudioStreamInfoBuilder {
    audio_stream_info: AudioStreamInfo,
}

impl AudioStreamInfoBuilder {
    pub fn audio_format(&mut self, audio_format: AudioFormat) -> &mut Self {
        self.audio_stream_info.audio_format = audio_format;
        self
    }

    pub fn compression_type(&mut self, ty: CompressionType) -> &mut Self {
        self.audio_stream_info.compression_type = ty;
        self
    }

    pub fn samples_per_frame(&mut self, samples_per_frame: u64) -> &mut Self {
        self.audio_stream_info.samples_per_frame = samples_per_frame;
        self
    }

    pub fn build(self) -> AudioStreamInfo {
        self.audio_stream_info
    }
}
