#![allow(unused_imports)]

use std::{
    collections::VecDeque,
    io::{BufReader, Read},
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    time::Duration,
};

use std::io;

struct AudioBuffer {
    rx: Receiver<Vec<u8>>,
    v: Vec<u8>,
}

impl Read for AudioBuffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.v.len();
        if len > 0 {
            let len = len.min(buf.len());
            let result: Vec<u8> = self.v.drain(..len).collect();
            buf[..len].copy_from_slice(&result);
            Ok(len)
        } else {
            match self.rx.recv() {
                Ok(mut v) => {
                    let len = v.len().min(buf.len());
                    let result: Vec<u8> = v.drain(..len).collect();
                    buf[..len].copy_from_slice(&result);
                    if !v.is_empty() {
                        self.v.extend(v);
                    }
                    Ok(len)
                }
                Err(err) => Err(io::Error::new(io::ErrorKind::InvalidData, err)),
            }
        }
    }
}

fn main() -> io::Result<()> {
    let (tx, rx) = mpsc::channel();
    let audio_buffer = AudioBuffer {
        v: vec![1, 2, 3, 4],
        rx,
    };
    let mut reader = BufReader::new(audio_buffer);
    let mut buf = [0; 512];
    let size = reader.read(&mut buf)?;
    println!("{:?}", &buf[..size]);

    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(5));
        tx.send(vec![33, 44, 55, 66]).unwrap();
    });

    let size = reader.read(&mut buf)?;
    println!("{:?}", &buf[..size]);

    Ok(())
}
