use crate::net::{request::Request, response::Response};

pub async fn handle(req: Request<'_>) -> anyhow::Result<Response> {
    log::info!("method = {:?} uri = {}", req.method(), req.uri());
    Ok(Response::rtsp_ok(&req))
}
