#![allow(non_snake_case)]

use num_bigint::{BigInt, Sign};

#[macro_export]
macro_rules! bnum_bytes_len {
    ($var:expr) => {
        ($var.bits() as usize + 7) / 8
    };
}

#[derive(Debug, Clone)]
pub struct NGConstant {
    pub N: BigInt,
    pub g: BigInt,
}

impl NGConstant {
    fn new_ng_custom(n_hex: &str, g_hex: &str) -> Self {
        Self {
            N: BigInt::from_bytes_be(Sign::Plus, &hex::decode(n_hex).unwrap()),
            g: BigInt::from_bytes_be(Sign::Plus, &hex::decode(g_hex).unwrap()),
        }
    }
}

pub mod airsrp;
