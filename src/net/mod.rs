use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Method {
    Get,
    Post,
    Put,
    Setup,
    GetParameter,
    Record,
    SetParameter,
    Teardown,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub enum Protocol {
    Rtsp1_0,
    Http1_1,
    Unknown,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Protocol::Rtsp1_0 => "RTSP/1.0".to_string(),
                Protocol::Http1_1 => "HTTP/1.1".to_string(),
                Protocol::Unknown => "Unknown".to_string(),
            }
        )
    }
}

pub mod request;
pub mod response;
pub mod server;
