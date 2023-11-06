use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    simplified_visual: bool,
    online_play: bool,
    critical_hits: bool,
    misses: bool,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            simplified_visual: false,
            online_play: false,
            critical_hits: false,
            misses: false,
        }
    }

    fn simplified_visual(mut self, value: bool) -> Self {
        self.simplified_visual = value;
        self
    }

    fn online_play(mut self, value: bool) -> Self {
        self.online_play = value;
        self
    }

    fn critical_hits(mut self, value: bool) -> Self {
        self.critical_hits = value;
        self
    }

    fn misses(mut self, value: bool) -> Self {
        self.misses = value;
        self
    }
}

fn main() {
    let my_settings = Settings::new()
        .simplified_visual(true)
        .online_play(true)
        .critical_hits(true)
        .misses(false);

    println!("{:?}", my_settings.simplified_visual);
}
