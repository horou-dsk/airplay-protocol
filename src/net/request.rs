use futures::future::BoxFuture;
use http::HeaderMap;
use tokio::{
    io::{self, AsyncReadExt, BufReader},
    net::TcpStream,
};

use super::{response::Response, Method, Protocol};

pub struct Body<'a> {
    len: usize,
    reader: BufReader<&'a mut TcpStream>,
}

impl<'a> Body<'a> {
    pub fn new(len: usize, reader: BufReader<&'a mut TcpStream>) -> Self {
        Self { len, reader }
    }

    pub async fn array(mut self) -> io::Result<Vec<u8>> {
        // self.reader.take(limit)
        if self.len == 0 {
            return Ok(vec![]);
        }
        let mut result = Vec::with_capacity(self.len);
        const BUF_LEN: usize = 512;
        let mut buf = [0; BUF_LEN];
        let mut len = self.len;
        loop {
            let amt = self.reader.read_exact(&mut buf[..len.min(BUF_LEN)]).await?;
            result.extend_from_slice(&buf[..amt]);
            if len <= buf.len() {
                break;
            }
            len -= amt;
        }
        Ok(result)
    }

    pub async fn text(self) -> io::Result<String> {
        let result = self.array().await?;
        Ok(String::from_utf8_lossy(&result).to_string())
    }

    pub async fn plist(self) -> io::Result<plist::Value> {
        let mut reader = self.reader.take(self.len as u64);
        let mut result = Vec::with_capacity(self.len);
        reader.read_to_end(&mut result).await?;
        let value: plist::Value = plist::from_bytes(&result).expect("plist der error");
        Ok(value)
    }
}

pub struct Request<'a> {
    method: Method,
    protocol: Protocol,
    uri: &'a str,
    body: Option<Body<'a>>,
    headers: HeaderMap,
    server_port: u16,
}

impl<'a> Request<'a> {
    pub fn new(
        method: Method,
        protocol: Protocol,
        uri: &'a str,
        body: Body<'a>,
        headers: HeaderMap,
        server_port: u16,
    ) -> Self {
        Self {
            method,
            protocol,
            uri,
            body: Some(body),
            headers,
            server_port,
        }
    }

    pub fn protocol(&self) -> Protocol {
        self.protocol
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn uri(&self) -> &str {
        self.uri
    }

    pub fn take_body(&mut self) -> Option<Body<'a>> {
        self.body.take()
    }

    pub fn into_body(self) -> Body<'a> {
        self.body.unwrap()
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn server_port(&self) -> u16 {
        self.server_port
    }
}

pub trait ServiceRequest: Sync + Send {
    fn call<'a>(&'a self, req: Request<'a>) -> BoxFuture<'a, anyhow::Result<Response>>;
}
