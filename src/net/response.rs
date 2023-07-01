use std::collections::HashMap;

use bytes::Bytes;
use http::StatusCode;

use super::{request::Request, Protocol};

type HeaderMapTy = HashMap<&'static str, String>;

#[derive(Debug, Clone)]
pub struct Response {
    protocol: Protocol,
    status: StatusCode,
    headers: HeaderMapTy,
    body: Option<Bytes>,
}

impl Response {
    pub fn rtsp_ok(req: &Request) -> Self {
        let mut headers = HashMap::new();
        headers.insert("server", "AirTunes/220.68".to_string());
        headers.insert("Content-Length", "0".to_string());
        if let Some(cseq) = req.headers().get("cseq") {
            headers.insert("cseq", cseq.to_str().unwrap().to_string());
        }
        Self {
            protocol: Protocol::Rtsp1_0,
            status: StatusCode::OK,
            headers,
            body: None,
        }
    }

    pub fn http_ok() -> Self {
        let mut headers = HashMap::new();
        headers.insert("server", "AirTunes/220.68".to_string());
        headers.insert("Content-Length", "0".to_string());
        Self {
            protocol: Protocol::Http1_1,
            status: StatusCode::OK,
            headers,
            body: None,
        }
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMapTy {
        &mut self.headers
    }

    pub fn headers(&self) -> &HeaderMapTy {
        &self.headers
    }

    pub fn into_bytes(self) -> Bytes {
        let mut result = Vec::new();
        let head = format!("{} {}\r\n", self.protocol, self.status);
        result.extend_from_slice(head.as_bytes());
        for (header_name, header_value) in self.headers.into_iter() {
            result.extend_from_slice(header_name.as_bytes());
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
        self.headers
            .insert("Content-Type", "text/html;charset=utf-8".to_string());
        let bytes = text.as_bytes();
        self.headers
            .insert("Content-Length", bytes.len().to_string());
        self.body = Some(Bytes::copy_from_slice(bytes));
        self
    }

    pub fn bytes_body(mut self, bytes: Bytes) -> Self {
        self.headers
            .insert("Content-Length", bytes.len().to_string());
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
