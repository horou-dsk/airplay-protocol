#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

// macro_rules! bnum_bytes_len {
//     ($var:expr) => {
//         ($var.bits() as usize + 7) / 8
//     };
// }

use airplay2_protocol::srp::{srp_create_salted_verification_key, SrpUesr, SrpVerifier};
use num_bigint::{BigInt, Sign};

fn main() {
    let username = "14:94:6C:6C:30:0A";
    let password = "2222";
    let (salt, verifier) = srp_create_salted_verification_key(username, password);
    let usr = SrpUesr::new(username, password);
    // println!("{:02X?}", verifier.to_bytes_be().1);
    let ver = SrpVerifier::new(username, &salt, verifier, &usr.A).unwrap();
    let M = BigInt::from_bytes_be(Sign::Plus, &ver.M);
    let usr = usr.process_challenge(&salt, &ver.B).unwrap();
    println!("{:?}", ver);
    println!("{:?}", usr);
}
