use bytes::Bytes;
use ed25519_dalek::PUBLIC_KEY_LENGTH;

use self::pairing::Pairing;

mod fairplay;
pub mod hand_garble;
mod pairing;

#[derive(Default)]
pub struct AirPlay {
    pairing: Pairing,
}

impl AirPlay {
    pub fn pair_setup(&self) -> [u8; PUBLIC_KEY_LENGTH] {
        self.pairing.pair_setup()
    }

    pub fn pair_verify(&mut self, data: &[u8]) -> Option<Bytes> {
        self.pairing.pair_verify(data)
    }
}
