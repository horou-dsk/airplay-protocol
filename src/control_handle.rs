use crate::net::{request::Request, response::Response, server::ResultResp, Protocol};

pub async fn handle(req: Request<'_>) -> ResultResp {
    log::info!(
        "method = {:?} uri = {} protocol = {}",
        req.method(),
        req.uri(),
        req.protocol()
    );
    log::info!("headers = {:?}", req.headers());
    let res = match req.protocol() {
        Protocol::Http1_1 => Response::http_ok().text_body("Hello World"),
        Protocol::Rtsp1_0 => Response::rtsp_ok(&req),
        Protocol::Unknown => Response::http_ok(),
    };
    Ok(res)
}
