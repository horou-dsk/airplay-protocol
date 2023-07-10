use futures::FutureExt;
use tokio::sync::Mutex;

use crate::{
    airplay::{
        airplay_consumer::ArcAirPlayConsumer,
        lib::media_stream_info::MediaStreamInfo,
        property_list,
        session::{ARSession, SessionManager},
        AirPlayConfig,
    },
    net::{
        request::{Request, ServiceRequest},
        response::Response,
        server::ResultResp,
        Method, Protocol,
    },
};

pub struct ControlHandle {
    airplay_config: AirPlayConfig,
    session_manager: Mutex<SessionManager>,
    audio_consumer: ArcAirPlayConsumer,
    video_consumer: ArcAirPlayConsumer,
}

impl ControlHandle {
    pub fn new(
        airplay_config: AirPlayConfig,
        video_consumer: ArcAirPlayConsumer,
        audio_consumer: ArcAirPlayConsumer,
    ) -> Self {
        Self {
            airplay_config,
            session_manager: Mutex::new(SessionManager::default()),
            audio_consumer,
            video_consumer,
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

    async fn remove_session(&self, req: &Request<'_>) -> Option<ARSession> {
        let session_id = req
            .headers()
            .get("Active-Remote")
            .or_else(|| req.headers().get("X-Apple-Session-ID"))
            .unwrap();
        self.session_manager
            .lock()
            .await
            .remove_session(session_id.to_str().unwrap())
    }

    async fn handle_get_info(&self, req: Request<'_>) -> ResultResp {
        let resp = Response::rtsp_ok(&req);
        let bytes = property_list::prepare_info_response(&self.airplay_config);
        Ok(resp.bytes_body(bytes))
    }

    async fn handle_pair_pin_start(&self, req: Request<'_>) -> ResultResp {
        log::info!("{:#?}", req.headers());
        let resp = Response::rtsp_ok(&req);
        Ok(resp)
    }

    async fn handle_pair_setup_pin(&self, req: Request<'_>) -> ResultResp {
        let resp = Response::rtsp_ok(&req);
        let session = self.resolve_session(&req).await;
        let data = req.into_body().array().await.expect("body read error");
        let data = session.airplay.write().await.pair_setup_pin(&data);
        if let Some(data) = data {
            Ok(resp.bytes_body(data))
        } else {
            Ok(resp)
        }
    }

    async fn handle_pair_setup(&self, req: Request<'_>) -> ResultResp {
        let session = self.resolve_session(&req).await;
        let key = session.airplay.read().await.pair_setup();
        Ok(Response::rtsp_ok(&req).slice_body(&key))
    }

    async fn handle_pair_verify(&self, req: Request<'_>) -> ResultResp {
        let session = self.resolve_session(&req).await;
        let resp = Response::rtsp_ok(&req);
        let data = req.into_body().array().await.expect("body read error");
        let data = session.airplay.write().await.pair_verify(&data);
        if let Some(data) = data {
            Ok(resp.bytes_body(data))
        } else {
            Ok(resp)
        }
    }

    async fn handle_fairplay_setup(&self, req: Request<'_>) -> ResultResp {
        let session = self.resolve_session(&req).await;
        let resp = Response::rtsp_ok(&req);
        let data = req.into_body().array().await.expect("body read error");
        let data = session.airplay.write().await.fairplay_setup(&data);
        if let Some(data) = data {
            Ok(resp.bytes_body(data))
        } else {
            Ok(resp)
        }
    }

    async fn handle_rtsp_setup(&self, req: Request<'_>) -> ResultResp {
        let session = self.resolve_session(&req).await;
        let resp = Response::rtsp_ok(&req);
        let server_port = req.server_port();
        let data = req.into_body().array().await.expect("body read error");
        let data = session.airplay.write().await.rstp_setup(&data);
        if let Some(data) = data {
            match data {
                MediaStreamInfo::Video(video) => {
                    self.video_consumer.on_video_format(video);
                    let mut video_server = session.video_server.write().await;
                    video_server
                        .start(
                            session.airplay.read().await.video_decryptor(),
                            self.video_consumer.clone(),
                        )
                        .await
                        .expect("start video server error!");
                    let setup = property_list::prepare_setup_video_response(
                        video_server.get_port(),
                        server_port,
                        0,
                    );
                    Ok(resp.bytes_body(setup))
                }
                MediaStreamInfo::Audio(audio) => {
                    self.audio_consumer.on_audio_format(audio);
                    let audio_server = &session.audio_server;
                    audio_server
                        .start(
                            session.airplay.read().await.audio_decryptor(),
                            self.audio_consumer.clone(),
                        )
                        .await
                        .expect("start audio server error!");
                    let setup = property_list::prepare_setup_audio_response(
                        audio_server.get_port().await,
                        server_port,
                    );
                    Ok(resp.bytes_body(setup))
                }
            }
        } else {
            Ok(resp)
        }
    }

    async fn handle_rtsp_get_parameter(&self, req: Request<'_>) -> ResultResp {
        Ok(Response::rtsp_ok(&req).slice_body(b"volume: 0.000000\r\n"))
    }

    async fn handle_rtsp_set_parameter(&self, mut req: Request<'_>) -> ResultResp {
        let mut resp = Response::rtsp_ok(&req);
        let body = req.take_body().unwrap();
        let content_type = req.headers().get("Content-Type");
        // TODO: application/x-dmap-tagged image/none image/jpeg ...
        let data = body.array().await;
        match (content_type, data) {
            (Some(header_value), Ok(data)) if header_value.as_bytes() == b"text/parameters" => {
                if data.len() >= 8 && b"volume: " == &data[..8] {
                    let volume = String::from_utf8_lossy(&data[8..data.len() - 2])
                        .parse::<f32>()
                        .unwrap_or(-20.0);
                    self.audio_consumer.on_volume(volume);
                }
            }
            _ => (),
        }
        resp.headers_mut()
            .insert("Audio-Jack-Status", "connected; type=analog".to_string());
        Ok(resp)
    }

    async fn hanlde_rtsp_teardown(&self, mut req: Request<'_>) -> ResultResp {
        let resp = Response::rtsp_ok(&req);
        let session = self.resolve_session(&req).await;
        let data = req
            .take_body()
            .unwrap()
            .array()
            .await
            .expect("body read error");
        let data = session.airplay.write().await.rstp_setup(&data);
        if let Some(media_info) = data {
            match media_info {
                MediaStreamInfo::Audio(_) => {
                    self.audio_consumer.on_audio_src_disconnect();
                    session.audio_server.stop().await;
                }
                MediaStreamInfo::Video(_) => {
                    self.video_consumer.on_video_src_disconnect();
                    session.video_server.write().await.stop();
                }
            }
        } else {
            self.remove_session(&req).await;
            // stop
        }
        Ok(resp)
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
                Protocol::Http1_1 => match (req.method(), req.uri()) {
                    (Method::Get, "/empty") => Ok(Response::http_ok()),
                    (Method::Get, "/info") => {
                        let mut resp = Response::http_ok().text_body(r#"{"a": 123}"#);
                        resp.headers_mut()
                            .insert("Content-Type", "application/json;".to_string());
                        Ok(resp)
                    }
                    (Method::Post, "/pair-pin-start") => self.handle_pair_pin_start(req).await,
                    (Method::Post, "/pair-setup-pin") => self.handle_pair_setup_pin(req).await,
                    _ => Ok(Response::http_ok().text_body("Hello World")),
                },
                Protocol::Rtsp1_0 => match (req.method(), req.uri()) {
                    (Method::Get, "/info") => self.handle_get_info(req).await,
                    (Method::Post, "/pair-pin-start") => self.handle_pair_pin_start(req).await,
                    (Method::Post, "/pair-setup-pin") => self.handle_pair_setup_pin(req).await,
                    (Method::Post, "/pair-setup") => self.handle_pair_setup(req).await,
                    (Method::Post, "/pair-verify") => self.handle_pair_verify(req).await,
                    (Method::Post, "/fp-setup") => self.handle_fairplay_setup(req).await,
                    (Method::Setup, _) => self.handle_rtsp_setup(req).await,
                    (Method::Post, "/feedback") => Ok(Response::rtsp_ok(&req)),
                    (Method::GetParameter, _) => self.handle_rtsp_get_parameter(req).await,
                    (Method::SetParameter, _) => self.handle_rtsp_set_parameter(req).await,
                    (Method::Teardown, _) => self.hanlde_rtsp_teardown(req).await,
                    _ => Ok(Response::rtsp_ok(&req)),
                },
                Protocol::Unknown => Ok(Response::http_ok()),
            };
            res
        }
        .boxed()
    }
}
