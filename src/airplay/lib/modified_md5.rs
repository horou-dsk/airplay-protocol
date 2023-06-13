#![allow(non_snake_case)]

use super::{get_i32_le, write_i32_le};

const SHIFT: [u8; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

#[inline]
fn rol(input: i64, count: i64) -> i64 {
    ((input << count) & 0xffffffff) | (input & 0xffffffff) >> (32 - count)
}

#[inline]
fn F(b: i64, c: i64, d: i64) -> i64 {
    (b & c) | (!b & d)
}

#[inline]
fn G(b: i64, c: i64, d: i64) -> i64 {
    (b & d) | (c & !d)
}

#[inline]
fn H(b: i64, c: i64, d: i64) -> i64 {
    b ^ c ^ d
}

#[inline]
fn I(b: i64, c: i64, d: i64) -> i64 {
    c ^ (b | !d)
}

#[inline]
fn swap_i32_le(buf: &mut [u8], a: usize, b: usize) {
    let size = 4;
    let tmp_a: [u8; 4] = buf[a..a + size].try_into().unwrap();
    let tmp_b: [u8; 4] = buf[b..b + size].try_into().unwrap();
    buf[a..a + size].copy_from_slice(&tmp_b);
    buf[b..b + size].copy_from_slice(&tmp_a);
}

pub(super) fn modified_md5(originalblock_in: &[u8], key_in: &[u8], key_out: &mut [u8]) {
    let mut block_in = [0; 64];
    let (mut a, mut b, mut c, mut d, mut z, mut tmp);

    block_in.copy_from_slice(&originalblock_in[..64]);

    a = get_i32_le(key_in, 0) as i64 & 0xffffffff;
    b = get_i32_le(key_in, 4) as i64 & 0xffffffff;
    c = get_i32_le(key_in, 8) as i64 & 0xffffffff;
    d = get_i32_le(key_in, 12) as i64 & 0xffffffff;

    for (i, s) in SHIFT.iter().enumerate() {
        let mut j = 0;
        if i < 16 {
            j = i;
        } else if i < 32 {
            j = (5 * i + 1) % 16;
        } else if i < 48 {
            j = (3 * i + 5) % 16;
        } else if i < 64 {
            j = 7 * i % 16;
        }
        let input = ((block_in[4 * j] as i32) << 24)
            | ((block_in[4 * j + 1] as i32) << 16)
            | ((block_in[4 * j + 2] as i32) << 8)
            | block_in[4 * j + 3] as i32;

        z = a + input as i64 + ((1i64 << 32) as f64 * ((i + 1) as f64).sin().abs()) as i64;

        if i < 16 {
            z = rol(z + F(b, c, d), *s as i64);
        } else if i < 32 {
            z = rol(z + G(b, c, d), *s as i64);
        } else if i < 48 {
            z = rol(z + H(b, c, d), *s as i64);
        } else if i < 64 {
            z = rol(z + I(b, c, d), *s as i64);
        }
        z += b;
        tmp = d;
        d = c;
        c = b;
        b = z;
        a = tmp;
        if i == 31 {
            // swapsies
            swap_i32_le(&mut block_in, 4 * (a & 15) as usize, 4 * (b & 15) as usize);
            swap_i32_le(&mut block_in, 4 * (c & 15) as usize, 4 * (d & 15) as usize);
            swap_i32_le(
                &mut block_in,
                4 * ((a & (15 << 4)) >> 4) as usize,
                4 * ((b & (15 << 4)) >> 4) as usize,
            );
            swap_i32_le(
                &mut block_in,
                4 * ((a & (15 << 8)) >> 8) as usize,
                4 * ((b & (15 << 8)) >> 8) as usize,
            );
            swap_i32_le(
                &mut block_in,
                4 * ((a & (15 << 12)) >> 12) as usize,
                4 * ((b & (15 << 12)) >> 12) as usize,
            );
            // swap(block_in, 4 * (a & 15) as i32, 4 * (b & 15) as i32);
            // swap(block_in, 4 * (c & 15) as i32, 4 * (d & 15) as i32);
            // swap(block_in, 4 * ((a & (15 << 4)) >> 4) as i32, 4 * ((b & (15 << 4)) >> 4) as i32);
            // swap(block_in, 4 * ((a & (15 << 8)) >> 8) as i32, 4 * ((b & (15 << 8)) >> 8) as i32);
            // swap(block_in, 4 * ((a & (15 << 12)) >> 12) as i32, 4 * ((b & (15 << 12)) >> 12) as i32);
        }
        write_i32_le(key_out, 0, get_i32_le(key_in, 0) + a as i32);
        write_i32_le(key_out, 4, get_i32_le(key_in, 4) + b as i32);
        write_i32_le(key_out, 8, get_i32_le(key_in, 8) + c as i32);
        write_i32_le(key_out, 12, get_i32_le(key_in, 12) + d as i32);
        // key_out[0..size].copy_from_slice(&(to_i32_le(&key_in[0..size]) + a as i32).to_le_bytes());
        // key_out[4..4 + size].copy_from_slice(&(to_i32_le(&key_in[4..4 + size]) + b as i32).to_le_bytes());
        // key_out[8..8 + size].copy_from_slice(&(to_i32_le(&key_in[8..8 + size]) + c as i32).to_le_bytes());
        // key_out[12..12 + size].copy_from_slice(&(to_i32_le(&key_in[12..12 + size]) + d as i32).to_le_bytes());
        // key_out.putInt((int) (key_words.getInt(0) + A));
        // key_out.putInt((int) (key_words.getInt(4) + B));
        // key_out.putInt((int) (key_words.getInt(8) + C));
        // key_out.putInt((int) (key_words.getInt(12) + D));
    }
}
