use std::{
    collections::{HashMap, HashSet},
    fmt::format,
    path::PathBuf,
    rc::Rc,
    sync::{Arc, Mutex},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use discord_rich_presence::{
    activity::{Activity, Assets, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use uuid::Uuid;

pub struct PresenceState {
    pub client: DiscordIpcClient,
    pub connected: bool,
    pub start: i64,
    pub commands_used: i32,
    pub sessions: HashMap<Uuid, Instant>,
    pub buttons: HashMap<String, String>,
    pub last_command: String,
}
impl PresenceState {
    pub fn new() -> Self {
        let buttons = HashMap::from([(
            "Shell Repo".to_string(),
            "https://github.com/MrPicklePinosaur/shrs".to_string(),
        )]);

        Self {
            client: DiscordIpcClient::new("1188721913586003988").unwrap(),

            start: 0,
            commands_used: 0,
            sessions: HashMap::new(),
            connected: false,
            buttons,
            last_command: String::new(),
        }
    }
    pub fn connect(&mut self, session: Uuid) {
        if self.sessions.len() == 0 {
            self.start = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64
        }
        self.sessions.insert(session, Instant::now());

        if !self.connected {
            self.connected = self.client.connect().is_ok();
        }
    }
    pub fn disconnect(&mut self) {
        self.connected = false;
        self.client.close().unwrap();
    }

    pub fn update_activity(&mut self) {
        if !self.connected {
            return;
        }

        let details = format!(
            "Sessions {}; Commands {};",
            self.sessions.len(),
            self.commands_used
        );
        let activity_buttons = self
            .buttons
            .iter()
            .map(|(k, v)| Button::new(k.as_str(), v.as_str()))
            .collect();
        self.client
            .set_activity(
                Activity::new()
                    .state(format!("> {}", self.last_command).as_str())
                    .details(details.as_str())
                    .timestamps(Timestamps::new().start(self.start))
                    .buttons(activity_buttons)
                    .assets(Assets::new().large_image("shrs-1024x1024")),
            )
            .unwrap();
    }
    pub fn clear_activity(&mut self) {
        self.client.clear_activity().unwrap();
    }
    pub fn drop_dead_sessions(&mut self) {
        self.sessions
            .retain(|_, v| v.elapsed() < Duration::from_secs(20));
    }
}
