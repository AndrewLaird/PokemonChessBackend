use std::collections::{HashMap, HashSet};
use tokio::sync::{broadcast, Mutex};


// Global Map available to all requests, to keep track of users
pub struct AppState {
    rooms: HashMap<String, RoomState>,
}

struct RoomState {
    users: Mutex<HashSet<String>>,
    tx: broadcast::Sender<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, name: String) {
        // wait for the lock to be released
        // then insert the room
        self.rooms.insert(name, RoomState::new());
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
