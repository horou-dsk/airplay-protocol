use std::{fs::File, io::BufReader, time::Duration};

use airplay2_protocol::setup_log;
use rodio::{Decoder, OutputStream, Source};

fn main() -> std::io::Result<()> {
    setup_log();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = File::open("./tmp/qd.m4a")?;
    let reader = BufReader::new(file);
    let source = Decoder::new(reader).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
    std::thread::sleep(Duration::from_secs(999));
    Ok(())
}
