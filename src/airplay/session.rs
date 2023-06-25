use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use super::{
    server::{audio_server::AudioServer, video_server::VideoServer},
    AirPlay,
};

pub struct Session {
    pub id: String,
    pub airplay: Arc<RwLock<AirPlay>>,
    pub video_server: Arc<RwLock<VideoServer>>,
    pub audio_server: AudioServer,
}

impl Session {
    pub fn new(id: String) -> Self {
        Self {
            id,
            airplay: Arc::new(RwLock::new(AirPlay::default())),
            video_server: Arc::new(RwLock::new(VideoServer::default())),
            audio_server: AudioServer::default(),
        }
    }
}

pub type ARSession = Arc<Session>;

#[derive(Default)]
pub struct SessionManager {
    sessions: HashMap<String, ARSession>,
}

impl SessionManager {
    pub fn get_session(&mut self, id: &str) -> ARSession {
        if let Some(session) = self.sessions.get_mut(id) {
            session.clone()
        } else {
            let session = Arc::new(Session::new(id.to_string()));
            self.sessions.insert(id.to_string(), session.clone());
            session
            // self.sessions.get_mut(id).unwrap()
        }
    }

    pub fn remove_session(&mut self, id: &str) -> Option<ARSession> {
        self.sessions.remove(id)
    }
}
