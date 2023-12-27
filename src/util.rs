use std::collections::BTreeMap;

use tokio::sync::Mutex;

pub struct AppState {
    pub client_v: Mutex<BTreeMap<String, String>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            client_v: Default::default(),
        }
    }
}
