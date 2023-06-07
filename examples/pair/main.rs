use curve25519_dalek::{edwards::CompressedEdwardsY, ristretto::CompressedRistretto, Scalar, constants::X25519_BASEPOINT};
use rand::rngs::OsRng;

fn main() {
    let buf = [
        1, 0, 0, 0, 248, 11, 74, 17, 224, 227, 170, 254, 233, 158, 138, 142, 239, 123, 175, 51, 23,
        204, 138, 50, 0, 253, 183, 147, 124, 215, 132, 254, 254, 71, 212, 88, 252, 90, 173, 150,
        153, 16, 161, 166, 4, 198, 203, 149, 149, 161, 158, 2, 165, 108, 125, 141, 18, 197, 85,
        252, 63, 213, 4, 101, 200, 60, 228, 74,
    ];
    // let buf = [
    //     1, 0, 0, 0, 213, 163, 102, 78, 225, 202, 240, 9, 155, 94, 82, 66, 83, 212, 246, 151, 136,
    //     46, 151, 210, 166, 36, 135, 71, 163, 82, 31, 108, 187, 153, 50, 14, 46, 66, 254, 103, 48,
    //     75, 248, 2, 123, 67, 221, 178, 109, 54, 25, 87, 38, 121, 227, 65, 17, 239, 191, 249, 45,
    //     60, 135, 244, 159, 241, 254, 79,
    // ];
    let bytes = &buf[4..];
    let mut rng = OsRng;
    let private_key = Scalar::random(&mut rng);
    let mut public_key = [0; 32];
    public_key.copy_from_slice(&bytes[..32]);
    let private_key = [72, 140, 39, 105, 130, 230, 82, 63, 52, 214, 253, 42, 75, 179, 64, 178, 77, 2, 211, 213, 195, 93, 109, 58, 253, 51, 3, 183, 29, 209, 33, 124];
    let private_key = Scalar::from_bits(private_key);
    let public_key = private_key * X25519_BASEPOINT;
    println!("{:?}", public_key.as_bytes());
    // let ecdh_theirs = CompressedRistretto::from_slice(&bytes[..32])
    //     .unwrap()
    //     .decompress()
    //     .unwrap();
    // let ecdh_theirs = CompressedEdwardsY::from_slice(&bytes[..32])
    //     .unwrap()
    //     .decompress()
    //     .unwrap();
    // let secret = their_public_key * private_key;

    // println!("{:?}", &bytes[..32]);
    // println!("{:?}", secret.as_bytes());
}
