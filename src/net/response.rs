use bytes::Bytes;
use hyper::{http::HeaderValue, HeaderMap, StatusCode};

use super::{request::Request, Protocol};

pub struct Response {
    protocol: Protocol,
    status: StatusCode,
    headers: HeaderMap,
    body: Option<Bytes>,
}

impl Response {
    pub fn rtsp_ok(req: &Request) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "server",
            HeaderValue::from_bytes(b"AirTunes/220.68").unwrap(),
        );
        headers.insert("Content-Length", HeaderValue::from_bytes(b"0").unwrap());
        if let Some(cseq) = req.headers().get("cseq") {
            headers.insert("cseq", cseq.clone());
        }
        Self {
            protocol: Protocol::Rtsp1_0,
            status: StatusCode::OK,
            headers,
            body: None,
        }
    }

    pub fn http_ok() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "server",
            HeaderValue::from_bytes(b"AirTunes/220.68").unwrap(),
        );
        headers.insert("Content-Length", HeaderValue::from_bytes(b"0").unwrap());
        Self {
            protocol: Protocol::Http1_1,
            status: StatusCode::OK,
            headers,
            body: None,
        }
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    pub fn into_bytes(self) -> Bytes {
        let mut result = Vec::new();
        let head = format!("{} {}\r\n", self.protocol, self.status);
        result.extend_from_slice(head.as_bytes());
        for (header_name, header_value) in self.headers.into_iter() {
            result.extend_from_slice(header_name.unwrap().as_str().as_bytes());
            result.extend_from_slice(b": ");
            result.extend_from_slice(header_value.as_bytes());
            result.extend_from_slice(b"\r\n");
        }
        result.extend_from_slice(b"\r\n");
        if let Some(body) = self.body {
            result.extend_from_slice(&body);
        }
        result.into()
    }

    pub fn text_body(mut self, text: &str) -> Self {
        self.headers.insert(
            "Content-Type",
            HeaderValue::from_static("text/html;charset=utf-8"),
        );
        let bytes = text.as_bytes();
        self.headers
            .insert("Content-Length", HeaderValue::from(bytes.len()));
        self.body = Some(Bytes::copy_from_slice(bytes));
        self
    }

    pub fn bytes_body(mut self, bytes: Bytes) -> Self {
        self.headers
            .insert("Content-Length", HeaderValue::from(bytes.len()));
        self.body = Some(bytes);
        self
    }

    pub fn slice_body(self, data: &[u8]) -> Self {
        self.bytes_body(Bytes::copy_from_slice(data))
    }

    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status = status_code;
        self
    }
}
