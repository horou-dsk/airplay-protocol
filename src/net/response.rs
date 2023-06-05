use hyper::{http::HeaderValue, HeaderMap, StatusCode};

use super::{request::Request, Protocol};

pub struct Response {
    protocol: Protocol,
    status: StatusCode,
    headers: HeaderMap,
}

impl Response {
    pub fn rtsp_ok(req: &Request) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "server",
            HeaderValue::from_bytes(b"AirTunes/220.68").unwrap(),
        );
        headers.insert("Content-Length", HeaderValue::from_bytes(b"0").unwrap());
        headers.insert("cseq", req.headers().get("cseq").unwrap().clone());
        Self {
            protocol: Protocol::Rtsp1_0,
            status: StatusCode::OK,
            headers,
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut result = Vec::new();
        let head = format!("{} {}\r\n", self.protocol, self.status);
        result.extend_from_slice(head.as_bytes());
        for (header_name, header_value) in self.headers.into_iter() {
            result.extend_from_slice(header_name.unwrap().as_str().as_bytes());
            result.extend_from_slice(b": ");
            result.extend_from_slice(header_value.as_bytes());
            result.extend_from_slice(b"\r\n");
        }
        result
    }
}
