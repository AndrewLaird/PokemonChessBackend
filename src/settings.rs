use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub local_play: bool,
    pub critical_hits: bool,
    pub misses: bool,
}

impl Settings {
    pub fn new(
        local_play: bool,
        critical_hits: bool,
        misses: bool,
    ) -> Self {
        Settings {
            local_play,
            critical_hits,
            misses,
        }
    }

    pub fn default() -> Self {
        Settings::new(false, false, false)
    }
}
