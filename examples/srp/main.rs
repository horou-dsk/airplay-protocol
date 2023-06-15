#![allow(unused_imports)]
#![allow(unused_variables)]
use std::ops::Deref;

use srp6::*;

// type Srp6Air = Srp6_2048;
type Srp6Air = Srp6<256, 16>;

type HandshakeProof16 = HandshakeProof<256, 16>;

struct AirSrp(Srp6Air);

impl Default for AirSrp {
    fn default() -> Self {
        Self(Srp6Air::new(
            Generator::from(7),
            "93BE8A2C0FAC7442480A9253539E32A6DE3C3F33D4B7DB4431344F41CAA975E28B626D23E553FCB1450850777ED260D2FFE1FB9816A6ED7164CD76D05733DE4EFA931514D008B7EA8A4BAC45AB7DFD8C346B924E04C37420EEAFCD486159FB49A236DC77B6884FBF3907F0AB8ED789692BA424C81E35A61A38C72EC3A7268B069FCBFBC236AA3167A11E1FD5CD1275021BAC8493CA3AAEBF4AEF685E93A0387C10861F8DB1C500D3DE1823D905EAB421D1E0FD92CEE61F44FF439D07388F1BA56DA112589878D565A199A3C27630DA8FAD31E07EE0A46269B302F215DD972CF9E746867F608DA4DA28A69399708FADC795A6B16276EB6EF5A90636D86DAF03A5"
                .try_into()
                .unwrap()
        ).unwrap())
    }
}

impl Deref for AirSrp {
    type Target = Srp6Air;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let username = "14:94:6C:6C:30:0A";
    let password = "2222";
    let salt = [
        118, 145, 47, 27, 248, 182, 243, 78, 229, 96, 69, 159, 115, 250, 149, 141,
    ];
    let salt = Salt::from_bytes_le(&salt);
    let key = [
        33, 198, 97, 207, 246, 2, 211, 88, 76, 231, 88, 177, 173, 31, 131, 58, 33, 116, 188, 212,
        169, 78, 21, 15, 148, 15, 150, 253, 155, 26, 193, 226, 32, 149, 183, 104, 202, 222, 137,
        177, 27, 91, 207, 77, 95, 65, 116, 41, 104, 212, 67, 199, 246, 14, 92, 188, 8, 86, 95, 186,
        201, 49, 143, 65, 167, 231, 214, 79, 109, 107, 139, 56, 161, 84, 106, 181, 196, 30, 242,
        112, 169, 120, 240, 199, 180, 60, 194, 79, 33, 217, 87, 51, 225, 241, 92, 161, 201, 30, 82,
        104, 107, 242, 132, 32, 134, 96, 50, 104, 150, 193, 187, 86, 39, 27, 79, 236, 144, 191,
        171, 190, 204, 157, 34, 70, 82, 217, 141, 84, 243, 235, 217, 171, 45, 156, 40, 45, 255, 17,
        246, 98, 22, 30, 152, 176, 121, 135, 11, 114, 158, 132, 39, 221, 115, 46, 253, 82, 233,
        162, 107, 221, 242, 88, 113, 29, 159, 175, 137, 182, 160, 64, 134, 159, 150, 198, 73, 168,
        220, 237, 190, 159, 25, 0, 157, 175, 173, 201, 56, 242, 188, 152, 52, 190, 156, 211, 131,
        7, 48, 197, 52, 198, 71, 178, 155, 134, 234, 65, 59, 129, 107, 12, 34, 54, 240, 66, 155,
        124, 135, 200, 163, 105, 159, 29, 254, 57, 120, 126, 119, 220, 110, 230, 238, 82, 11, 97,
        63, 109, 32, 40, 251, 94, 122, 12, 224, 99, 248, 218, 165, 142, 166, 140, 115, 147, 81,
        146, 14, 78,
    ];
    let verifier = PasswordVerifier::from_bytes_le(&key);

    // let (salt, verifier) = AirSrp::default().generate_new_user_secrets(username, password);
    // println!("{:?}", salt.to_vec());
    // println!("{:?}", verifier.to_vec());
    let user = UserDetails {
        username: username.to_string(),
        salt,
        verifier,
    };
    // let user = mock
    let (handshake, proof_verifier) = AirSrp::default().start_handshake(&user);
    let (proof, strong_proof_verifier) = handshake
        .calculate_proof(user.username.as_str(), password)
        .unwrap();
    println!(" - Proof          [M1] = {:?}", &proof.M1.to_vec());
    println!(" - Proof          [A] = {:?}", &proof.A.to_vec());
    // let pk = [
    //     199, 213, 49, 152, 41, 69, 206, 19, 15, 21, 129, 169, 212, 253, 49, 10, 77, 59, 175, 233,
    //     128, 204, 211, 57, 179, 230, 215, 19, 164, 126, 168, 9, 123, 164, 42, 122, 119, 120, 249,
    //     248, 18, 168, 218, 177, 87, 227, 122, 104, 31, 134, 135, 137, 213, 141, 0, 139, 206, 126,
    //     11, 40, 250, 134, 189, 203, 146, 175, 110, 245, 147, 17, 101, 230, 106, 232, 83, 62, 169,
    //     122, 215, 104, 65, 29, 167, 183, 93, 157, 36, 175, 180, 94, 32, 118, 176, 80, 231, 35, 76,
    //     182, 212, 12, 4, 234, 27, 101, 48, 5, 227, 84, 101, 49, 87, 188, 235, 161, 143, 144, 32,
    //     197, 94, 191, 52, 200, 176, 118, 174, 41, 223, 65, 34, 158, 214, 82, 146, 79, 237, 149,
    //     244, 29, 56, 94, 77, 120, 159, 156, 169, 177, 162, 255, 213, 107, 239, 60, 84, 243, 170,
    //     184, 52, 172, 36, 76, 139, 217, 211, 99, 192, 229, 252, 17, 155, 21, 135, 233, 180, 208,
    //     28, 250, 81, 186, 239, 92, 178, 50, 47, 237, 19, 216, 225, 205, 125, 77, 205, 245, 177, 93,
    //     50, 37, 57, 54, 219, 150, 118, 141, 49, 119, 240, 99, 239, 132, 231, 51, 230, 6, 46, 155,
    //     17, 232, 132, 147, 199, 108, 85, 177, 60, 208, 5, 172, 114, 233, 59, 226, 149, 212, 109,
    //     122, 181, 201, 142, 213, 84, 17, 145, 217, 167, 79, 29, 62, 251, 134, 242, 130, 210, 136,
    //     203, 2, 38, 142,
    // ];
    // let proof = [
    //     82, 197, 37, 167, 94, 219, 166, 127, 121, 115, 20, 221, 63, 246, 36, 139, 120, 139, 6, 132,
    // ];
    // let a = PublicKey::from_bytes_be(&pk);
    // let m1 = Proof::from_bytes_be(&proof);
    // let proof = HandshakeProof16 { A: a, M1: m1 };
    let strong_proof = proof_verifier.verify_proof(&proof);
    match strong_proof {
        Ok((strong_proof, session_key_server)) => {
            println!(" - Strong Proof     [M2] = {:?}", &strong_proof.to_vec());
            println!(
                " - Session Key      [K]  = {:?}",
                &session_key_server.to_vec()
            );
        }
        Err(err) => {
            eprintln!("{:?}", err);
        }
    }
}
