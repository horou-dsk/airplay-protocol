#![allow(non_snake_case)]

use num_bigint::{BigInt, RandBigInt, Sign, ToBigInt};
use rand::Rng;
use sha1::{Digest, Sha1};

#[derive(Debug)]
pub struct NGConstant {
    N: BigInt,
    g: BigInt,
}

impl NGConstant {
    fn new_ng() -> Self {
        let n_hex = "AC6BDB41324A9A9BF166DE5E1389582FAF72B6651987EE07FC3192943DB56050A37329CBB4\
A099ED8193E0757767A13DD52312AB4B03310DCD7F48A9DA04FD50E8083969EDB767B0CF60\
95179A163AB3661A05FBD5FAAAE82918A9962F0B93B855F97993EC975EEAA80D740ADBF4FF\
747359D041D5C33EA71D281E446B14773BCA97B43A23FB801676BD207A436C6481F1D2B907\
8717461A5B9D32E688F87748544523B524B0D57D5EA77A2775D2ECFA032CFBDBF52FB37861\
60279004E57AE6AF874E7303CE53299CCC041C7BC308D82A5698F3A8D0C38271AE35F8E9DB\
FBB694B5C803D89F7AE435DE236D525F54759B65E372FCD68EF20FA7111F9E4AFF73";
        let g_hex = "02";
        Self::new_ng_custom(n_hex, g_hex)
    }

    fn new_ng_custom(n_hex: &str, g_hex: &str) -> Self {
        Self {
            N: BigInt::from_bytes_be(Sign::Plus, &hex::decode(n_hex).unwrap()),
            g: BigInt::from_bytes_be(Sign::Plus, &hex::decode(g_hex).unwrap()),
        }
    }
}

fn hash_bytes(b: &[u8]) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.update(b);
    hasher.finalize().to_vec()
}

fn hash_num(n: &BigInt) -> [u8; 20] {
    let mut hasher = Sha1::new();
    hasher.update(&n.to_bytes_be().1);
    hasher.finalize().into()
}

fn h_ns(n: &BigInt, bytes: &[u8]) -> BigInt {
    let mut bin = n.to_bytes_be().1;
    bin.extend_from_slice(bytes);
    let mut hasher = Sha1::new();
    hasher.update(&bin);
    let result = hasher.finalize();
    BigInt::from_bytes_be(Sign::Plus, &result)
}

fn h_nn(n1: &BigInt, n2: &BigInt) -> BigInt {
    let mut bin = n1.to_bytes_be().1;
    bin.extend(n2.to_bytes_be().1);
    let mut hasher = Sha1::new();
    hasher.update(&bin);
    let result = hasher.finalize();
    BigInt::from_bytes_be(Sign::Plus, &result)
}

fn calculate_x(salt: &BigInt, username: &[u8], password: &[u8]) -> BigInt {
    let mut hasher = Sha1::new();
    hasher.update(username);
    hasher.update(b":");
    hasher.update(password);
    let ucp_hash = hasher.finalize();
    h_ns(salt, &ucp_hash)
}

pub fn srp_create_salted_verification_key(username: &str, password: &str) -> (BigInt, BigInt) {
    // let v = I2048::ZERO;
    // let x = I2048::ZERO;
    let ng = NGConstant::new_ng();
    let salt = rand::thread_rng().gen::<[u8; 16]>();
    let s = BigInt::from_bytes_be(Sign::Plus, &salt);
    let x = calculate_x(&s, username.as_bytes(), password.as_bytes());
    println!(
        "x = {:02X?}\n ng->g = {:02X?}\n ng->N = {:02X?}",
        x.to_bytes_be().1,
        ng.g.to_bytes_be().1,
        ng.N.to_bytes_be().1
    );
    let v = ng.g.modpow(&x, &ng.N);
    (s, v)
}

// fn srp_verifier_start_authentication(v: &BigInt) {
//     let ng = NGConstant::new_ng();
//     let b = rand::thread_rng().gen_biguint(256).to_bigint().unwrap();
// }

fn calculate_M(
    ng: &NGConstant,
    username: &str,
    s: &BigInt,
    A: &BigInt,
    B: &BigInt,
    K: &[u8],
) -> [u8; 20] {
    let H_N = hash_num(&ng.N);
    let H_g = hash_num(&ng.g);
    let H_I = hash_bytes(username.as_bytes());
    let mut H_xor = [0; 20];
    (0..H_xor.len()).for_each(|i| H_xor[i] = H_N[i] ^ H_g[i]);

    let mut hasher = Sha1::new();
    hasher.update(H_xor);
    hasher.update(&H_I);
    hasher.update(&s.to_bytes_be().1);
    hasher.update(&A.to_bytes_be().1);
    hasher.update(&B.to_bytes_be().1);
    hasher.update(K);
    hasher.finalize().into()
}

fn calculate_H_AMK(A: &BigInt, M: &[u8], K: &[u8]) -> [u8; 20] {
    let mut hasher = Sha1::new();
    hasher.update(&A.to_bytes_be().1);
    hasher.update(M);
    hasher.update(K);
    hasher.finalize().into()
}

#[derive(Debug)]
pub struct SrpVerifier {
    pub M: [u8; 20],
    pub H_AMK: [u8; 20],
    pub session_key: [u8; 20],
    pub B: BigInt,
}

impl SrpVerifier {
    pub fn new(username: &str, salt: &BigInt, verifier: BigInt, A: &BigInt) -> Option<Self> {
        let ng = NGConstant::new_ng();
        let mut tmp1 = A % &ng.N;
        if tmp1 != 0.into() {
            let b = rand::thread_rng().gen_biguint(256).to_bigint().unwrap();
            let k = h_nn(&ng.N, &ng.g);
            tmp1 = k * &verifier;
            let tmp2 = ng.g.modpow(&b, &ng.N);
            let B = (tmp1 + tmp2) % &ng.N;

            let u = h_nn(A, &B);

            tmp1 = verifier.modpow(&u, &ng.N);
            let tmp2 = A * tmp1;
            let S = tmp2.modpow(&b, &ng.N);

            let session_key = hash_num(&S);
            let M = calculate_M(&ng, username, salt, A, &B, &session_key);
            let H_AMK = calculate_H_AMK(A, &M, &session_key);
            Some(Self {
                M,
                H_AMK,
                session_key,
                B,
            })
        } else {
            None
        }
    }
}

// fn srp_verifier_new(username: &str, salt: BigInt, verifier: BigInt, A: BigInt) {

// }

#[derive(Debug)]
pub struct Step2User {
    pub session_key: [u8; 20],
    pub M: [u8; 20],
    pub H_AMK: [u8; 20],
}

#[derive(Debug)]
pub struct SrpUesr {
    pub ng: NGConstant,
    pub a: BigInt,
    pub A: BigInt,
    pub username: String,
    pub password: String,
}

impl SrpUesr {
    pub fn new(username: &str, password: &str) -> Self {
        let ng = NGConstant::new_ng();
        let a = rand::thread_rng().gen_biguint(256).to_bigint().unwrap();
        let A = ng.g.modpow(&a, &ng.N);
        Self {
            username: username.to_string(),
            password: password.to_string(),
            A,
            a,
            ng,
        }
    }

    pub fn process_challenge(&self, salt: &BigInt, B: &BigInt) -> Option<Step2User> {
        let u = h_nn(&self.A, B);
        let x = calculate_x(salt, self.username.as_bytes(), self.password.as_bytes());
        let k = h_nn(&self.ng.N, &self.ng.g);
        let zero = 0.into();
        if B != &zero && u != zero {
            let v = self.ng.g.modpow(&x, &self.ng.N);
            let tmp1 = &u * &x;
            let tmp2 = tmp1 + &self.a;
            let tmp1 = v;
            let tmp3 = &k * tmp1;
            let tmp1 = B - tmp3;
            let S = tmp1.modpow(&tmp2, &self.ng.N);

            let session_key = hash_num(&S);

            let M = calculate_M(&self.ng, &self.username, salt, &self.A, B, &session_key);
            let H_AMK = calculate_H_AMK(&self.A, &M, &session_key);

            Some(Step2User {
                session_key,
                H_AMK,
                M,
            })
        } else {
            None
        }
    }
}
