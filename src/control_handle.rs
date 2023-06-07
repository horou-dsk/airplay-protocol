use futures::FutureExt;
use tokio::sync::Mutex;

use crate::{
    airplay::{
        property_list,
        session::{ARSession, SessionManager},
        AirPlayConfig,
    },
    net::{
        request::{Request, ServiceRequest},
        response::Response,
        server::ResultResp,
        Protocol,
    },
};

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

pub struct ControlHandle {
    airplay_config: AirPlayConfig,
    session_manager: Mutex<SessionManager>,
}

impl ControlHandle {
    pub fn new(airplay_config: AirPlayConfig) -> Self {
        Self {
            airplay_config,
            session_manager: Mutex::new(SessionManager::default()),
        }
    }

    async fn resolve_session(&self, req: &Request<'_>) -> ARSession {
        let session_id = req
            .headers()
            .get("Active-Remote")
            .or_else(|| req.headers().get("X-Apple-Session-ID"))
            .unwrap();
        self.session_manager
            .lock()
            .await
            .get_session(session_id.to_str().unwrap())
    }

    async fn handle_get_info(&self, req: Request<'_>) -> ResultResp {
        let resp = Response::rtsp_ok(&req);
        let bytes = property_list::prepare_info_response(&self.airplay_config);
        Ok(resp.bytes_body(bytes))
    }

    async fn handle_pair_setup(&self, req: Request<'_>) -> ResultResp {
        let session = self.resolve_session(&req).await;
        let key = session.read().await.airplay.pair_setup();
        Ok(Response::rtsp_ok(&req).slice_body(&key))
    }

    async fn handle_pair_verify(&self, req: Request<'_>) -> ResultResp {
        let session = self.resolve_session(&req).await;
        let resp = Response::rtsp_ok(&req);
        let data = req.into_body().array().await.expect("body read error");
        let data = session.write().await.airplay.pair_verify(&data);
        if let Some(data) = data {
            Ok(resp.bytes_body(data))
        } else {
            Ok(resp)
        }
    }
}

impl ServiceRequest for ControlHandle {
    fn call<'a>(&'a self, req: Request<'a>) -> futures::future::BoxFuture<'a, ResultResp> {
        async move {
            // log::info!(
            //     "method = {:?} uri = {} protocol = {}",
            //     req.method(),
            //     req.uri(),
            //     req.protocol()
            // );

            // log::info!("headers = {:?}", req.headers());
            let res = match req.protocol() {
                Protocol::Http1_1 => match req.uri() {
                    "/empty" => Ok(Response::http_ok()),
                    _ => Ok(Response::http_ok().text_body("Hello World")),
                },
                Protocol::Rtsp1_0 => match req.uri() {
                    "/info" => self.handle_get_info(req).await,
                    "/pair-setup" => self.handle_pair_setup(req).await,
                    "/pair-verify" => self.handle_pair_verify(req).await,
                    _ => Ok(Response::rtsp_ok(&req)),
                },
                Protocol::Unknown => Ok(Response::http_ok()),
            };
            res
        }
        .boxed()
    }
}
