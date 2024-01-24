pub struct AirPlayConfig {
    pub server_name: String,
    pub width: u32,
    pub height: u32,
    pub fps: u16,
    pub volume: f32,
    pub audio_buffer_size: Option<u16>,
    pub pin_pwd: Option<String>,
}

pub struct AirPlayConfigBuilder {
    config: AirPlayConfig,
}

impl AirPlayConfigBuilder {
    pub fn new(server_name: String) -> Self {
        Self {
            config: AirPlayConfig {
                server_name,
                width: 1920,
                height: 1080,
                fps: 60,
                volume: 0.5,
                audio_buffer_size: None,
                pin_pwd: None,
            },
        }
    }

    pub fn width(mut self, width: u32) -> Self {
        self.config.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.config.height = height;
        self
    }

    pub fn volume(mut self, volume: f32) -> Self {
        self.config.volume = volume;
        self
    }

    pub fn fps(mut self, fps: u16) -> Self {
        self.config.fps = fps;
        self
    }

    pub fn audio_buffer_size(mut self, audio_buffer_size: u16) -> Self {
        self.config.audio_buffer_size = Some(audio_buffer_size);
        self
    }

    pub fn pin_pwd<S: Into<String>>(mut self, pin_pwd: S) -> Self {
        self.config.pin_pwd = Some(pin_pwd.into());
        self
    }

    pub fn build(self) -> AirPlayConfig {
        self.config
    }
}

pub mod airplay_consumer;
pub mod lib;
pub use lib::AirPlay;
pub mod property_list;
pub mod server;
pub mod session;
