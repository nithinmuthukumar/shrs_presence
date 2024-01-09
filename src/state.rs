use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use discord_rich_presence::{
    activity::{Activity, Assets, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};

pub struct PresenceState {
    pub client: DiscordIpcClient,
    pub connected: bool,
    pub start: i64,
    pub commands_used: i32,
    pub working_dir: PathBuf,
}
impl PresenceState {
    pub fn new() -> Self {
        Self {
            client: DiscordIpcClient::new("1188721913586003988").unwrap(),
            start: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
            commands_used: 0,
            working_dir: PathBuf::new(),
            connected: false,
        }
    }
    pub fn connect(&mut self) -> bool {
        self.connected = self.client.connect().is_ok();

        self.connected
    }
    pub fn disconnect(&mut self) {
        self.connected = false;
        self.client.close();
    }

    pub fn update_activity(&mut self, state: &str) {
        if !self.connected {
            return;
        }
        let home = std::env::var("HOME").unwrap();
        let mut wd = self.working_dir.clone();

        if let Ok(p) = wd.strip_prefix(home) {
            wd = PathBuf::from("~").join(p);
        }

        let details = format!("📁 {}", wd.to_string_lossy());
        self.client
            .set_activity(
                Activity::new()
                    .state(state)
                    .details(details.as_str())
                    .timestamps(Timestamps::new().start(self.start))
                    .buttons(vec![Button::new(
                        "Shell Repo",
                        "https://github.com/nithinmuthukumar",
                    )])
                    .assets(Assets::new().large_image("shrs-1024x1024")),
            )
            .unwrap();
    }
    pub fn clear_activity(&mut self) {
        self.client.clear_activity();
    }
}
