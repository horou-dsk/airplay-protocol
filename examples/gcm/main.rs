use aes::{
    cipher::{generic_array::GenericArray, typenum::U16},
    Aes128,
};
use aes_gcm::aead::Aead;
use sha2::{Digest, Sha512};

fn main() {
    use aes_gcm::{AesGcm, KeyInit};
    type Aes128Gcm = AesGcm<Aes128, U16>;
    let session_key = [
        164, 95, 16, 67, 233, 46, 234, 70, 94, 177, 246, 63, 226, 229, 182, 227, 131, 165, 209,
        107, 7, 222, 166, 239, 142, 93, 79, 0, 68, 128, 138, 224, 48, 208, 153, 64, 103, 10, 23,
        84,
    ];
    let mut hasher = Sha512::new();
    hasher.update("Pair-Setup-AES-Key".as_bytes());
    hasher.update(session_key);

    let mut aes_key = [0u8; 16];
    aes_key.copy_from_slice(&hasher.finalize_reset()[..16]);

    hasher.update("Pair-Setup-AES-IV".as_bytes());
    hasher.update(session_key);
    let mut aes_iv = [0u8; 16];
    aes_iv.copy_from_slice(&hasher.finalize_reset()[..16]);
    aes_iv[15] += 1;

    let epk = [
        234, 107, 245, 98, 44, 180, 75, 189, 81, 146, 17, 142, 59, 45, 96, 156, 205, 233, 172, 76,
        177, 211, 127, 192, 134, 108, 173, 19, 178, 117, 201, 39,
    ];
    let auth_tag = [
        9, 247, 195, 41, 171, 15, 71, 41, 119, 21, 228, 186, 8, 10, 64, 235,
    ];

    let cipher = Aes128Gcm::new(&aes_key.into());

    let iv = GenericArray::from_slice(&aes_iv);

    let data = [
        46, 66, 254, 103, 48, 75, 248, 2, 123, 67, 221, 178, 109, 54, 25, 87, 38, 121, 227, 65, 17,
        239, 191, 249, 45, 60, 135, 244, 159, 241, 254, 79,
    ];

    let result = cipher.encrypt(iv, data.as_ref());

    println!("{:?}", result);
}
