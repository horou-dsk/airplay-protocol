pub struct AirPlayConfig {
    pub server_name: String,
    pub width: u32,
    pub height: u32,
    pub fps: u16,
}
pub mod airplay_consumer;
pub mod lib;
pub use lib::AirPlay;
pub mod property_list;
pub mod server;
pub mod session;
