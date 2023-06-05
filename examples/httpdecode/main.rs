use std::str::FromStr;

use httparse::EMPTY_HEADER;
use hyper::{
    http::{HeaderName, HeaderValue},
    HeaderMap,
};

fn main() {
    let b = br#"GET_PARAMETER rtsp://192.169.1.19/15994942520421728915 RTSP/1.0
Content-Length: 8
Content-Type: text/parameters
CSeq: 8
DACP-ID: 3A5C7B2F4762308C
Active-Remote: 2154041534
User-Agent: AirPlay/675.4.1
"#;
    let mut header = [EMPTY_HEADER; 2];
    if let Err(err) = httparse::parse_headers(b, &mut header) {
        eprintln!("{err:?}");
    }
    let mut header_map = HeaderMap::new();
    header_map.insert(
        HeaderName::from_str("Content-Length").unwrap(),
        HeaderValue::from(100),
    );
    println!(
        "{:?}",
        header_map.get(HeaderName::from_bytes(b"content-length").unwrap())
    );
    println!("{:?}", header);
}
