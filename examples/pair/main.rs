use aes::cipher::{KeyIvInit, StreamCipher};
use curve25519_dalek::{constants::X25519_BASEPOINT, Scalar};
use ed25519_dalek::{Signer, SigningKey};
use sha2::{Digest, Sha512};

type Aes128Ctr64LE = ctr::Ctr64BE<aes::Aes128>;

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
    // let mut rng = OsRng;
    // let private_key = Scalar::random(&mut rng);
    let mut ecdh_theirs = [0; 32];
    ecdh_theirs.copy_from_slice(&bytes[..32]);
    let ecdh_theirs = curve25519_dalek::montgomery::MontgomeryPoint(ecdh_theirs);
    let private_key = [
        24, 193, 45, 159, 96, 174, 247, 67, 65, 0, 232, 4, 231, 76, 215, 30, 120, 194, 88, 235, 70,
        50, 45, 122, 216, 0, 99, 239, 195, 51, 140, 83,
    ];
    let private_key = Scalar::from_bits(private_key);
    let public_key = private_key * X25519_BASEPOINT;
    let ecdh_ours = public_key.as_bytes();
    println!("{:?}", public_key.as_bytes());
    let secret = ecdh_theirs * private_key;
    println!(
        "{:?}",
        secret
            .as_bytes()
            .iter()
            .map(|v| format!("{:02x}", v))
            .collect::<String>()
    );

    fn init_cipher(ecdh_secret: &[u8]) -> Aes128Ctr64LE {
        let mut hasher = Sha512::new();
        hasher.update("Pair-Verify-AES-Key".as_bytes());
        hasher.update(ecdh_secret);

        let mut shared_secret_sha512_aes_key = [0u8; 16];
        shared_secret_sha512_aes_key.copy_from_slice(&hasher.finalize()[..16]);

        println!("{:?}", shared_secret_sha512_aes_key);

        let mut hasher = Sha512::new();
        hasher.update("Pair-Verify-AES-IV".as_bytes());
        hasher.update(ecdh_secret);

        let mut shared_secret_sha512_aes_iv = [0u8; 16];
        shared_secret_sha512_aes_iv.copy_from_slice(&hasher.finalize()[..16]);

        Aes128Ctr64LE::new(
            &shared_secret_sha512_aes_key.into(),
            &shared_secret_sha512_aes_iv.into(),
        )
    }

    let mut cipher = init_cipher(secret.as_bytes());

    let mut data_to_sign = [0; 64];
    data_to_sign[..32].copy_from_slice(ecdh_ours);
    data_to_sign[32..].copy_from_slice(ecdh_theirs.as_bytes());

    // let mut signature = [
    //     200, 160, 150, 17, 135, 193, 185, 147, 103, 14, 82, 195, 228, 66, 184, 232, 137, 152, 65,
    //     119, 103, 182, 215, 253, 97, 133, 15, 156, 176, 154, 15, 217, 62, 37, 148, 137, 135, 138,
    //     104, 207, 215, 20, 152, 109, 191, 125, 215, 94, 179, 176, 136, 35, 219, 208, 240, 42, 242,
    //     79, 144, 124, 243, 147, 16, 2,
    // ];
    // println!("{:?}", signature);

    // cipher.apply_keystream(&mut signature);

    let secret_key = [
        12, 149, 232, 179, 197, 182, 181, 89, 153, 201, 121, 207, 201, 126, 111, 217, 110, 56, 254,
        50, 85, 5, 146, 201, 190, 104, 106, 82, 65, 94, 149, 135,
    ];

    let signing_key = SigningKey::from_bytes(&secret_key);

    println!("public key = {:?}", signing_key.verifying_key().as_bytes());

    let signature = signing_key.sign(&data_to_sign);

    println!("data_to_sign = {:?}", data_to_sign);
    println!("signature = {:?}", signature.to_vec());

    let mut encrypted_signature = signature.to_vec();
    cipher.apply_keystream(&mut encrypted_signature);

    let mut result = Vec::new();
    result.extend_from_slice(ecdh_ours);
    result.extend_from_slice(&encrypted_signature);

    println!("result = {:?}", result);

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
