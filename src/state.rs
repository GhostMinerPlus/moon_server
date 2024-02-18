//! Global state of server

use std::collections::BTreeMap;

use tokio::sync::Mutex;

pub struct ServerState {
    pub client_v: Mutex<BTreeMap<String, String>>,
}

impl Default for ServerState {
    fn default() -> Self {
        Self {
            client_v: Default::default(),
        }
    }
}
