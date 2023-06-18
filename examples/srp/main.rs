#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

// macro_rules! bnum_bytes_len {
//     ($var:expr) => {
//         ($var.bits() as usize + 7) / 8
//     };
// }

use airplay2_protocol::srp::{
    airsrp::{AirSrp, NgType},
    srp_create_salted_verification_key, SrpUesr, SrpVerifier,
};
use ed25519::signature::Keypair;
use ed25519_dalek::SigningKey;
use num_bigint::{BigInt, Sign};

fn main() {
    // let username = "14:94:6C:6C:30:0A";
    // let password = "2222";
    // let (salt, verifier) = srp_create_salted_verification_key(username, password);
    // let usr = SrpUesr::new(username, password);
    // // println!("{:02X?}", verifier.to_bytes_be().1);
    // let ver = SrpVerifier::new(username, &salt, verifier, &usr.A).unwrap();
    // let M = BigInt::from_bytes_be(Sign::Plus, &ver.M);
    // let usr = usr.process_challenge(&salt, &ver.B).unwrap();
    // println!("{:?}", ver);
    // println!("{:?}", usr);
    let step1_resp = hex::decode("62706c6973743030d201020304566d6574686f6454757365725370696e5f101037354642454543373733434643353633080d14191d0000000000000101000000000000000500000000000000000000000000000030").unwrap();
    let plist_value: plist::Value = plist::from_bytes(&step1_resp).unwrap();
    println!("{:?}", plist_value);
    let air_srp = AirSrp::new(NgType::SrpNg2048, "14:94:6C:6C:30:0A", "2222");
    let mut csprng = rand::rngs::OsRng;
    let keypair = SigningKey::generate(&mut csprng);
    let verifying_key = [
        148, 140, 116, 70, 235, 132, 110, 197, 163, 146, 134, 240, 233, 46, 115, 130, 131, 154, 87,
        243, 168, 252, 191, 204, 203, 14, 112, 173, 80, 26, 117, 59,
    ]; //keypair.verifying_key().to_bytes();
    let handshake = air_srp.create_salted_verification_key(verifying_key);
    // (air_srp.pad() * handshake.v) + air_srp.ng.g.modpow(exponent, modulus)
    // println!("{:?}", air_srp.pad());
    println!("{:?}", handshake.pk_B.to_bytes_be().1);
    // println!("salt = {:?}", handshake.salt.to_bytes_be().1);
    // println!("pkB = {:?}", handshake.pk_B.to_bytes_be().1);
    let ver = handshake.new_verifier(&[
        22, 117, 94, 64, 34, 220, 167, 45, 167, 145, 205, 220, 122, 161, 191, 190, 44, 245, 252,
        118, 96, 20, 202, 106, 124, 175, 228, 148, 62, 166, 131, 91, 233, 192, 89, 62, 2, 12, 199,
        193, 119, 194, 166, 36, 245, 175, 115, 73, 248, 125, 189, 39, 72, 191, 55, 82, 249, 186,
        185, 169, 209, 65, 62, 236, 8, 105, 153, 143, 167, 58, 176, 230, 181, 31, 103, 172, 215,
        57, 113, 11, 239, 137, 218, 15, 98, 133, 162, 213, 212, 14, 84, 211, 142, 21, 9, 250, 137,
        114, 206, 188, 78, 162, 51, 164, 12, 172, 252, 211, 179, 68, 1, 71, 202, 134, 115, 85, 113,
        9, 86, 213, 163, 112, 63, 63, 168, 85, 202, 238, 145, 247, 66, 22, 36, 17, 209, 98, 80,
        186, 46, 84, 183, 69, 115, 242, 157, 127, 40, 148, 126, 18, 217, 190, 40, 172, 233, 189,
        204, 249, 83, 185, 202, 11, 195, 235, 252, 95, 233, 62, 187, 158, 215, 193, 175, 163, 20,
        248, 177, 23, 211, 237, 76, 67, 175, 70, 43, 172, 103, 131, 147, 122, 50, 151, 30, 144,
        141, 216, 192, 87, 140, 180, 160, 19, 9, 13, 165, 38, 110, 107, 99, 27, 248, 37, 241, 233,
        114, 207, 104, 231, 21, 141, 133, 141, 86, 18, 65, 25, 80, 175, 166, 203, 164, 85, 208,
        220, 92, 40, 171, 108, 207, 122, 159, 35, 128, 52, 231, 160, 93, 172, 110, 128, 108, 219,
        224, 222, 178, 110,
    ]);
    println!("ver = {:?}", ver);
}
