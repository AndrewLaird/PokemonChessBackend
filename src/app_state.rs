use std::collections::{HashMap, HashSet};
use tokio::sync::{broadcast, Mutex};

// Global Map available to all requests, to keep track of users
pub struct AppState {
    rooms: Mutex<HashMap<String, RoomState>>,
}

struct RoomState {
    users: Mutex<HashSet<String>>,
    tx: broadcast::Sender<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            rooms: Mutex::new(HashMap::new()),
        }
    }
}

impl RoomState {
    fn new() -> Self {
        Self {
            users: Mutex::new(HashSet::new()),
            tx: broadcast::channel(69).0,
        }
    }

}
