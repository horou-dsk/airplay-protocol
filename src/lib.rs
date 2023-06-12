#![feature(test)]
#![feature(fn_traits)]
#![feature(async_fn_in_trait)]

pub mod airplay;
pub mod airplay_bonjour;
pub mod control_handle;
pub mod net;

extern "C" {
    pub fn foo(a: u8, b: u8) -> u8;
    pub fn print_buf(buf: *const u8, len: usize);
}
