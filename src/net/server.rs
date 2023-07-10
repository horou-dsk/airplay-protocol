use std::{net::SocketAddr, sync::Arc, time::Duration};

use http::{HeaderMap, HeaderName, HeaderValue};
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
    pub handle: Arc<Box<dyn ServiceRequest>>,
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

async fn decoder(
    mut stream: TcpStream,
    handle: Arc<Box<dyn ServiceRequest>>,
    server_port: u16,
) -> io::Result<()> {
    log::info!("连接进入....");
    // let mut index = 0;
    loop {
        // index += 1;
        // log::info!("index ========== {index}");

        let mut reader = BufReader::new(&mut stream);
        let mut initial_line = String::new();
        let amt = if let Ok(r) =
            tokio::time::timeout(Duration::from_secs(60), reader.read_line(&mut initial_line)).await
        {
            r?
        } else {
            break;
        };
        if amt == 0 {
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
        while reader.read_line(&mut header_line).await? > 2 {}
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
                let resp_bytes = resp.into_bytes();
                stream.write_all(&resp_bytes).await?;
                // log::info!("resp = \n{}", String::from_utf8_lossy(&resp_bytes));
                stream.flush().await?;
            }
            Err(err) => {
                log::error!("{err:?}");
                break;
            }
        }
    }
    log::info!("连接断开....");
    Ok(())
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
            handle: Arc::new(Box::new(handle)),
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
            handle: Arc::new(Box::new(handle)),
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
