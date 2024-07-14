use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    simplified_visual: bool,
    online_play: bool,
    critical_hits: bool,
    misses: bool,
}

impl Settings {
    pub fn new(
        simplified_visual: bool,
        online_play: bool,
        critical_hits: bool,
        misses: bool,
    ) -> Self {
        Settings {
            simplified_visual,
            online_play,
            critical_hits,
            misses,
        }
    }
}
