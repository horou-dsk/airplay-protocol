use crate::net::{request::Request, response::Response, server::ResultResp};

pub async fn handle(req: Request<'_>) -> ResultResp {
    log::info!("method = {:?} uri = {}", req.method(), req.uri());
    async {}.await;
    Ok(Response::rtsp_ok(&req))
}
