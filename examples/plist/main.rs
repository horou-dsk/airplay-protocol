use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Streams {
    timestamp_info: Vec<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Setup {
    streams: Vec<Streams>,
}

fn main() {
    // let value: Setup = plist::from_file("./tmp/p.plist").expect("failed to read p.plist");
    // println!("{:?}", value);
    let value1 = hex::decode("62706c6973743030d20102030457617574685461675365706b4f101052a92f8712c6ea417f3adb3d03d8e5634f1020ff07fc8520d10728e6f2ab0a0245dfa20709b5d1ae5f9a19328b0663ba9414f2080d15192c000000000000010100000000000000050000000000000000000000000000004f").unwrap();
    let value2 = hex::decode("62706c6973743030d2010203045365706b57617574685461674f10206285b20afad4cefe1fce40cee685ab072c75240cb47fb71bc3b3d03dca52dc5d4f1010893eb8e5ae418b245e9b1bf7cba9116b080d11193c000000000000010100000000000000050000000000000000000000000000004f").unwrap();

    let client: plist::Dictionary = plist::from_bytes(&value1).unwrap();
    let server: plist::Dictionary = plist::from_bytes(&value2).unwrap();

    println!("client = {:?}", client);
    println!("server = {:?}", server);
}
