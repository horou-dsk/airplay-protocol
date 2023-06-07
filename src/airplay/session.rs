use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use super::AirPlay;

pub struct Session {
    pub id: String,
    pub airplay: AirPlay,
}

impl Session {
    pub fn new(id: String) -> Self {
        Self {
            id,
            airplay: AirPlay::default(),
        }
    }
}

pub type ARSession = Arc<RwLock<Session>>;

#[derive(Default)]
pub struct SessionManager {
    sessions: HashMap<String, ARSession>,
}

impl SessionManager {
    pub fn get_session(&mut self, id: &str) -> ARSession {
        if let Some(session) = self.sessions.get_mut(id) {
            session.clone()
        } else {
            let session = Arc::new(RwLock::new(Session::new(id.to_string())));
            self.sessions.insert(id.to_string(), session.clone());
            session
            // self.sessions.get_mut(id).unwrap()
        }
    }
}
