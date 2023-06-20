#![allow(non_snake_case)]

use ed25519_dalek::PUBLIC_KEY_LENGTH;
use num_bigint::{BigInt, RandBigInt, Sign, ToBigInt};
use sha1::{Digest, Sha1 as SrpSha};

use crate::bnum_bytes_len;

use super::NGConstant;

macro_rules! bint_to_bytes {
    ($var:expr) => {
        ($var.to_bytes_be().1)
    };
}

macro_rules! bytes_to_bint {
    ($var:expr) => {
        num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, $var)
    };
}

// type HashDest = [u8; 64];
type HashDest = [u8; 20];

macro_rules! hash_many_bytes {
    ($($var:expr),*) => {{
        let mut hasher = SrpSha::new();
        $(hasher.update($var);)*
        HashDest::from(hasher.finalize())
    }};
}

#[derive(Debug)]
pub struct AirSrp {
    pub ng: NGConstant,
    pub username: String,
    pub password: String,
}

pub enum NgType {
    SrpNg2048,
    SrpNg3072,
}

#[inline]
fn rand_bigint(bit: u64) -> BigInt {
    rand::thread_rng().gen_biguint(bit).to_bigint().unwrap()
}

fn hash_bytes(b: &[u8]) -> HashDest {
    let mut hasher = SrpSha::new();
    hasher.update(b);
    hasher.finalize().into()
}

fn hash_num(n: &BigInt) -> HashDest {
    let mut hasher = SrpSha::new();
    hasher.update(&n.to_bytes_be().1);
    hasher.finalize().into()
}

fn H_nn_pad(n1: &BigInt, n2: &BigInt, padded_len: usize) -> BigInt {
    let len_n1 = bnum_bytes_len!(n1);
    let len_n2 = bnum_bytes_len!(n2);
    let nbytes = 2 * padded_len;
    let offset_n1 = padded_len - len_n1;
    let offset_n2 = nbytes - len_n2;

    let mut bin = vec![0u8; nbytes];
    bin[offset_n1..offset_n1 + len_n1].copy_from_slice(&n1.to_bytes_be().1);
    bin[offset_n2..offset_n2 + len_n2].copy_from_slice(&n2.to_bytes_be().1);
    let buff = hash_bytes(&bin);
    BigInt::from_bytes_be(Sign::Plus, &buff)
}

fn h_ns(n: &BigInt, bytes: &[u8]) -> BigInt {
    let mut bin = n.to_bytes_be().1;
    bin.extend_from_slice(bytes);
    let mut hasher = SrpSha::new();
    hasher.update(&bin);
    let result = hasher.finalize();
    BigInt::from_bytes_be(Sign::Plus, &result)
}

fn calculate_x(salt: &BigInt, username: &[u8], password: &[u8]) -> BigInt {
    let mut hasher = SrpSha::new();
    hasher.update(username);
    hasher.update(b":");
    hasher.update(password);
    let ucp_hash = hasher.finalize();
    h_ns(salt, &ucp_hash)
}

fn calculate_M(
    ng: &NGConstant,
    username: &str,
    s: &BigInt,
    A: &BigInt,
    B: &BigInt,
    K: &[u8],
) -> HashDest {
    let H_N = hash_num(&ng.N);
    let H_g = hash_num(&ng.g);
    let H_I = hash_bytes(username.as_bytes());
    let mut H_xor = [0; 20];
    (0..H_xor.len()).for_each(|i| H_xor[i] = H_N[i] ^ H_g[i]);

    let mut hasher = SrpSha::new();
    hasher.update(H_xor);
    hasher.update(H_I);
    hasher.update(&s.to_bytes_be().1);
    hasher.update(&A.to_bytes_be().1);
    hasher.update(&B.to_bytes_be().1);
    hasher.update(K);
    hasher.finalize().into()
}

fn calculate_H_AMK(A: &BigInt, M: &[u8], K: &[u8]) -> HashDest {
    let mut hasher = SrpSha::new();
    hasher.update(&A.to_bytes_be().1);
    hasher.update(M);
    hasher.update(K);
    hasher.finalize().into()
}

impl AirSrp {
    pub fn new(ng_ty: NgType, username: &str, password: &str) -> Self {
        let (n_hex, g_hex) = match ng_ty {
            NgType::SrpNg2048 => (
                "AC6BDB41324A9A9BF166DE5E1389582FAF72B6651987EE07FC3192943DB56050A37329CBB4\
A099ED8193E0757767A13DD52312AB4B03310DCD7F48A9DA04FD50E8083969EDB767B0CF60\
95179A163AB3661A05FBD5FAAAE82918A9962F0B93B855F97993EC975EEAA80D740ADBF4FF\
747359D041D5C33EA71D281E446B14773BCA97B43A23FB801676BD207A436C6481F1D2B907\
8717461A5B9D32E688F87748544523B524B0D57D5EA77A2775D2ECFA032CFBDBF52FB37861\
60279004E57AE6AF874E7303CE53299CCC041C7BC308D82A5698F3A8D0C38271AE35F8E9DB\
FBB694B5C803D89F7AE435DE236D525F54759B65E372FCD68EF20FA7111F9E4AFF73",
                "02",
            ),
            NgType::SrpNg3072 => (
                "FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B\
139B22514A08798E3404DDEF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245E485\
B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7EDEE386BFB5A899FA5AE9F24117C4B1F\
E649286651ECE45B3DC2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F83655D23\
DCA3AD961C62F356208552BB9ED529077096966D670C354E4ABC9804F1746C08CA18217C32\
905E462E36CE3BE39E772C180E86039B2783A2EC07A28FB5C55DF06F4C52C9DE2BCBF69558\
17183995497CEA956AE515D2261898FA051015728E5A8AAAC42DAD33170D04507A33A85521\
ABDF1CBA64ECFB850458DBEF0A8AEA71575D060C7DB3970F85A6E1E4C7ABF5AE8CDB0933D7\
1E8C94E04A25619DCEE3D2261AD2EE6BF12FFA06D98A0864D87602733EC86A64521F2B1817\
7B200CBBE117577A615D6C770988C0BAD946E208E24FA074E5AB3143DB5BFCE0FD108E4B82\
D120A93AD2CAFFFFFFFFFFFFFFFF",
                "05",
            ),
        };
        Self {
            ng: NGConstant::new_ng_custom(n_hex, g_hex),
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    pub fn pad(&self, v: &[u8]) -> Vec<u8> {
        let padded_len = bnum_bytes_len!(self.ng.N);
        let mut padded = vec![0; padded_len];
        padded[padded_len - v.len()..padded_len].copy_from_slice(v);
        padded
    }

    pub fn create_salted_verification_key(self, public_key: [u8; PUBLIC_KEY_LENGTH]) -> Handshake {
        let ng = &self.ng;
        let s = rand_bigint(128);
        let x = calculate_x(&s, self.username.as_bytes(), self.password.as_bytes());
        let v = ng.g.modpow(&x, &ng.N);
        let (b, pk_B) = self.verifier_start_authentication(&v, Some(&public_key));
        Handshake {
            ng: self.ng.clone(),
            username: self.username.clone(),
            air_srp: self,
            public_key,
            pk_B,
            b,
            salt: s,
            v,
        }
    }

    pub fn verifier_start_authentication(
        &self,
        v: &BigInt,
        secret_key: Option<&[u8]>,
    ) -> (BigInt, BigInt) {
        let ng = &self.ng;
        let b = secret_key
            .map(|v| bytes_to_bint!(v))
            .unwrap_or_else(|| rand_bigint(256));
        let k = H_nn_pad(&self.ng.N, &self.ng.g, bnum_bytes_len!(self.ng.N));

        let tmp1 = k * v;
        let tmp2 = ng.g.modpow(&b, &ng.N);
        let B = (tmp1 + tmp2) % &ng.N;

        (b, B)
    }
}

#[derive(Debug)]
pub struct Handshake {
    air_srp: AirSrp,
    pub pk_B: BigInt,
    pub b: BigInt,
    pub salt: BigInt,
    pub v: BigInt,
    pub username: String,
    pub ng: NGConstant,
    pub public_key: [u8; PUBLIC_KEY_LENGTH],
}

impl Handshake {
    pub fn new_verifier(&self, A: &[u8]) -> Verifier {
        let ng = &self.ng;
        let B = &self.pk_B;
        let u = bytes_to_bint!(&hash_many_bytes!(
            self.air_srp.pad(A),
            self.air_srp.pad(&B.to_bytes_be().1)
        ));
        let A = bytes_to_bint!(A);
        let tmp = &A * self.v.modpow(&u, &ng.N);
        let s = tmp.modpow(&self.b, &ng.N) % &ng.N;

        let session_key = {
            let s = bint_to_bytes!(s);
            let hash1 = hash_many_bytes!(&s, [0, 0, 0, 0]);
            let hash2 = hash_many_bytes!(&s, [0, 0, 0, 1]);
            [&hash1[..], &hash2[..]].concat()
        };

        let m1 = calculate_M(ng, &self.username, &self.salt, &A, B, &session_key);
        let m2 = calculate_H_AMK(&A, &m1, &session_key);

        Verifier {
            M1: m1,
            M2: m2,
            session_key,
        }
    }

    // Homekit ?
    /* pub fn new_verifier(self, A: &[u8]) -> Verifier {
        let ng = &self.ng;
        let client_pk = bytes_to_bint!(A);
        let pad_a = bint_to_bytes!(self.air_srp.pad(&client_pk));
        let server_pk = bytes_to_bint!(&self.public_key());
        let pad_b = bint_to_bytes!(self.air_srp.pad(&server_pk));
        let pad_ab = [&pad_a[..], &pad_b[..]].concat();
        let common_secret = bytes_to_bint!(&hash_bytes(&pad_ab));
        let tmp1 = self.v.modpow(&common_secret, &ng.N);
        let tmp2 = client_pk * tmp1;
        let pbk = bytes_to_bint!(&self.public_key);
        let premaster_secret = tmp2.modpow(&pbk, &ng.N);
        let session_key = hash_num(&premaster_secret);

        let proof = {
            let t1 =
                bint_to_bytes!(bytes_to_bint!(&hash_num(&ng.N)) ^ bytes_to_bint!(&hash_num(&ng.g)));
            let t2 = hash_bytes(self.username.as_bytes());
            let t3 = bint_to_bytes!(self.salt);
            let t4 = A;
            let t5 = bint_to_bytes!(server_pk);
            let result = [&t1[..], &t2[..], &t3[..], t4, &t5[..], &session_key[..]].concat();
            hash_bytes(&result)
        };
        let key_proof_hash = hash_many_bytes!(A, &proof, &session_key);

        Verifier {
            key_proof_hash,
            proof,
            handshake: self,
        }
    } */

    pub fn public_key(&self) -> HashDest {
        let ng = &self.ng;
        let pad = H_nn_pad(&self.ng.N, &self.ng.g, bnum_bytes_len!(self.ng.N));
        let pk = bytes_to_bint!(&self.public_key);
        let public_key = ((pad * &self.v) + (ng.g.modpow(&pk, &ng.N))) % &ng.N;
        hash_bytes(&bint_to_bytes!(public_key))
    }
}

#[derive(Debug)]
pub struct Verifier {
    pub M1: HashDest,
    pub M2: HashDest,
    pub session_key: Vec<u8>,
}
// Homekit?
// #[derive(Debug)]
// pub struct Verifier {
//     pub key_proof_hash: HashDest,
//     pub proof: HashDest,
//     pub handshake: Handshake,
// }
