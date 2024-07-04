use std::{net::SocketAddr, sync::Arc, time::Duration};

use http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

use super::{
    request::{Body, Request, ServiceRequest},
    response::Response,
    Method, Protocol,
};

pub type ResultResp = anyhow::Result<Response>;

pub struct Server {
    listener: TcpListener,
    pub handle: Arc<dyn ServiceRequest>,
    pub port: u16,
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

async fn decoder(mut stream: TcpStream, handle: Arc<dyn ServiceRequest>, server_port: u16) {
    log::info!("连接进入....");
    let mut normal_disconnect = false;
    'out: loop {
        let mut reader = BufReader::new(&mut stream);
        let mut initial_line = String::new();
        let amt = tokio::select! {
            result = reader.read_line(&mut initial_line) => {
                match result {
                    Ok(amt) => amt,
                    Err(err) => {
                        log::error!("read_line error = {:?}", err);
                        break;
                    }
                }
            },
            _ = tokio::time::sleep(Duration::from_secs(10)) => {
                let result = tokio::time::timeout(Duration::from_secs(3), stream.write_all(b"ping")).await;
                if result.is_err() || result.unwrap().is_err() {
                    break;
                } else {
                    continue;
                }
            }
        };
        if amt == 0 {
            normal_disconnect = true;
            break;
        }
        let methods: Vec<&str> = initial_line.split(' ').collect();
        log::info!("methods = {:?}", methods);
        if methods.len() != 3 {
            continue;
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
            "RTSP/1.0\r\n" => Protocol::Rtsp1_0,
            "HTTP/1.1\r\n" => Protocol::Http1_1,
            _ => Protocol::Unknown,
        };
        let uri = methods[1];
        let mut header_line = String::new();
        loop {
            match reader.read_line(&mut header_line).await {
                Ok(size) if size > 2 => (),
                Ok(_) => break,
                Err(_) => break 'out,
            }
        }
        let headers = parse_header(&header_line);
        let content_length: usize = headers
            .get("content-length")
            .map(|v| v.to_str().unwrap_or("0").parse().unwrap_or(0))
            .unwrap_or(0);

        let body = Body::new(content_length, reader);
        let request = Request::new(method, protocol, uri, body, headers, server_port);
        let resp = handle.call(request).await;

        match resp {
            Ok(resp) => {
                if resp.status() == StatusCode::INTERNAL_SERVER_ERROR {
                    break;
                }
                let resp_bytes = resp.into_bytes();
                let _ = stream.write_all(&resp_bytes).await;
                // log::info!("resp = \n{}", String::from_utf8_lossy(&resp_bytes));
                let _ = stream.flush().await;
            }
            Err(err) => {
                log::error!("{err:?}");
                break;
            }
        }
    }
    if !normal_disconnect {
        handle.disconnect().await;
    }
    log::info!("连接断开....");
}

impl Server {
    pub async fn bind_with_addr<T>(addr: SocketAddr, handle: T) -> Self
    where
        T: ServiceRequest,
        T: 'static,
    {
        let port = addr.port();
        let listener = TcpListener::bind(addr).await.unwrap();
        Self {
            port,
            listener,
            handle: Arc::new(handle),
        }
    }

    pub async fn bind_default<T>(handle: T) -> Self
    where
        T: ServiceRequest,
        T: 'static,
    {
        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        Self {
            port,
            listener,
            handle: Arc::new(handle),
        }
    }

    pub async fn run(self) -> io::Result<()> {
        let listener = self.listener;
        loop {
            let (stream, _) = listener.accept().await?;
            tokio::task::spawn(decoder(stream, self.handle.clone(), self.port));
        }
    }
}
