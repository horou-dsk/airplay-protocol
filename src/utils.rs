use std::fmt::Write;

pub fn encode_hex(buf: &[u8]) -> String {
    let mut s = String::new();
    for byte in buf {
        write!(s, "{:02X}", byte).expect("Unable to write to string");
    }
    s
}
