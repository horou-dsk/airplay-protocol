#![allow(unused_imports)]
#![allow(unused_variables)]
use std::{ops::Deref, time::Duration};

use airplay2_protocol::airplay::property_list;
use srp6::*;

// type Srp6Air = Srp6_2048;
type Srp6Air = Srp6<256, 256>;

type HandshakeProof16 = HandshakeProof<256, 256>;

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
    let (salt, verifier) = AirSrp::default().generate_new_user_secrets(username, password);
    let user = UserDetails {
        verifier,
        salt,
        username: username.to_string(),
    };
    let (handshake, proof_verifier) = AirSrp::default().start_handshake(&user);
    let (proof, strong_proof_verifier) = handshake.calculate_proof(username, password).unwrap();
    print!("{:#?}", proof);
    let a = proof.A.to_vec();
    let m1 = proof.M1.to_vec();
    let proof = HandshakeProof16 {
        A: PublicKey::from_bytes_le(&a),
        M1: Proof::from_bytes_le(&m1),
    };
    print!("{:#?}", proof);
    std::thread::sleep(Duration::from_secs(5));
    println!("{:?}", proof_verifier.verify_proof(&proof));
}
