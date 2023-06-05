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
    let value: Setup = plist::from_file("./tmp/p.plist").expect("failed to read p.plist");
    println!("{:?}", value);
}
