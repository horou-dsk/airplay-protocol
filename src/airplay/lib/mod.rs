use bytes::Bytes;
use ed25519_dalek::PUBLIC_KEY_LENGTH;

use self::pairing::Pairing;

// 将字节数组按照小端字节序转换为 u32
fn to_i32_le(bytes: &[u8]) -> i32 {
    i32::from_le_bytes(bytes.try_into().unwrap())
}

mod fairplay;
pub mod hand_garble;
mod modified_md5;
mod omg_hax;
mod omg_hax_const;
mod pairing;
mod sap_hash;

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
