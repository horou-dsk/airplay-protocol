#![feature(test)]
#![feature(fn_traits)]
#![feature(async_fn_in_trait)]
use std::io::Write;

pub fn setup_log() {
    let mut builder = env_logger::Builder::new();
    let env = env_logger::Env::default().default_filter_or("info");
    // builder.parse_filters("libmdns=debug");
    builder.parse_env(env);
    builder.format(|buf, record| {
        let mut style = buf.style();
        style.set_bold(true);
        match record.level() {
            log::Level::Error => {style.set_color(env_logger::fmt::Color::Red);},
            log::Level::Warn => {style.set_color(env_logger::fmt::Color::Yellow);},
            log::Level::Info => {style.set_color(env_logger::fmt::Color::Green);},
            _ => ()
            // log::Level::Debug => todo!(),
            // log::Level::Trace => todo!(),
        };
        writeln!(
            buf,
            "[{} {} {}:{}] {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            style.value(record.level()),
            record.module_path().unwrap_or("unknown"),
            record.line().unwrap_or(0),
            record.args()
        )
    });
    builder.init();
}

pub mod airplay;
pub mod airplay_bonjour;
pub mod control_handle;
pub mod net;
pub mod srp;
mod utils;

extern "C" {
    pub fn foo(a: u8, b: u8) -> u8;
    pub fn print_buf(buf: *const u8, len: usize);
}
