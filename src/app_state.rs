use std::collections::{HashMap, HashSet};
use tokio::sync::{broadcast, Mutex};


// Global Map available to all requests, to keep track of users
pub struct AppState {
    pub rooms: HashMap<String, RoomState>,
}

struct RoomState {
    pub  users: HashSet<String>,
    // tx means transmitter, we send messages to everyone in the room
    // because each user in the room has the other end of the tx ( a rx )
    // created via tx.subscribe()
    // they are all waiting for messages to come out of their rx to send to their client
    // and when their client sends a message, we write it into the tx so all other clients 
    // in the room can receive it
    pub tx: broadcast::Sender<String>,
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

    pub fn get_room_tx(&mut self, name: &str) -> broadcast::Sender<String> {
        if let Some(room) = self.rooms.get(name) {
            return room.tx.clone();
        }
        // room not found, create a new one
        self.add_room(name.to_string());
        return self.get_room_tx(name);
    }
}

impl RoomState {
    fn new() -> Self {
        Self {
            users: HashSet::new(),
            tx: broadcast::channel(69).0,
        }
    }

}
