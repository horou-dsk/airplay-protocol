use std::{future::Future, net::SocketAddr};

use hyper::{
    http::{HeaderName, HeaderValue},
    HeaderMap,
};
use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

use super::{
    request::{Body, Request},
    response::Response,
    Method, Protocol,
};

pub type ResultResp = anyhow::Result<Response>;

pub trait Handler: Clone + 'static {
    type Output;
    type Future: Future<Output = Self::Output>;

    fn call(&self, req: Request) -> Self::Future;

    fn fuck(&self);
}

impl<F, Fut> Handler for F
where
    F: Fn(Request) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future,
{
    type Output = Fut::Output;

    type Future = Fut;

    fn call(&self, req: Request) -> Self::Future {
        (self)(req)
    }

    fn fuck(&self) {
        println!("Fuck!!!!!!!!!");
    }
}

// type ServiceFn<F, Fut> = Handler<>;

pub struct Server<F> {
    pub addr: SocketAddr,
    pub service: F,
}

fn parse_header(header_str: &str) -> HeaderMap {
    let lines = header_str.lines();
    let mut map = HeaderMap::new();
    for line in lines {
        if let Some((key, value)) = line.split_once(':') {
            let value = if &value[..1] == " " {
                &value[1..]
            } else {
                value
            };
            map.insert(
                HeaderName::from_bytes(key.as_bytes()).unwrap(),
                HeaderValue::from_str(value).unwrap(),
            );
        }
    }
    map
}

async fn decoder<F>(mut stream: TcpStream, service: F) -> io::Result<()>
where
    F: Handler,
{
    loop {
        let mut reader = BufReader::new(&mut stream);
        let mut initial_line = String::new();
        reader.read_line(&mut initial_line).await?;
        let methods: Vec<&str> = initial_line.split(' ').collect();
        if methods.len() != 3 {
            // continue;
        }
        let method = match methods[0] {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "SETUP" => Method::Setup,
            "GET_PARAMETER" => Method::GetParameter,
            "SET_PARAMETER" => Method::SetParameter,
            "TEARDOWN" => Method::Teardown,
            "RECORD" => Method::Record,
            _ => Method::Unknown,
        };
        let protocol = match methods[2] {
            "RTSP/1.0" => Protocol::Rtsp1_0,
            "HTTP/1.1" => Protocol::Http1_1,
            _ => Protocol::Unknown,
        };
        let uri = methods[1];
        let mut header_line = String::new();
        while reader.read_line(&mut header_line).await? > 2 {}
        let headers = parse_header(&header_line);
        let content_length: usize = headers
            .get("content-length")
            .map(|v| v.to_str().unwrap().parse().unwrap())
            .unwrap_or(0);
        let body = Body::new(content_length, reader);
        let request = Request::new(method, protocol, uri.to_string(), body, headers);
        let resp = crate::control_handle::handle.call((request,)).await;
        // let resp = service.call(request).await;

        match resp {
            Ok(resp) => {
                stream.write_all(&resp.into_bytes()).await?;
            }
            Err(err) => {
                log::error!("{err:?}");
                break;
            }
        }
        // let mut header_map = HashMap::new();
        // header_map.append("key", value)
    }
    Ok(())
}

impl<F> Server<F>
where
    F: Handler + std::marker::Send,
{
    pub fn bind(addr: SocketAddr, f: F) -> Self {
        Self { addr, service: f }
    }

    pub async fn run(self) -> io::Result<()> {
        let listener = TcpListener::bind(self.addr).await?;
        loop {
            let (stream, _) = listener.accept().await?;
            let service = self.service.clone();
            tokio::task::spawn(decoder(stream, service));
        }
    }
}
