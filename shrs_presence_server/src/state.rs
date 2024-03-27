use std::{
    collections::HashMap,
    fmt::format,
    path::PathBuf,
    rc::Rc,
    sync::{Arc, Mutex},
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
    pub sessions: u32,
    pub buttons: HashMap<String, String>,
}
impl PresenceState {
    pub fn new() -> Self {
        let buttons = HashMap::from([(
            "Shell Repo".to_string(),
            "https://github.com/MrPicklePinosaur/shrs".to_string(),
        )]);

        Self {
            client: DiscordIpcClient::new("1188721913586003988").unwrap(),

            start: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
            commands_used: 0,
            sessions: 0,
            connected: false,
            buttons,
        }
    }
    pub fn connect(&mut self) -> bool {
        self.connected = self.client.connect().is_ok();
        dbg!(self.connected);

        self.connected
    }
    pub fn disconnect(&mut self) {
        self.connected = false;
        self.client.close();
    }

    pub fn update_activity(&mut self) {
        if !self.connected {
            return;
        }

        let details = format!(
            "Sessions {}; Commands {};",
            self.sessions, self.commands_used
        );
        let state = format!("cd ~");
        let activity_buttons = self
            .buttons
            .iter()
            .map(|(k, v)| Button::new(k.as_str(), v.as_str()))
            .collect();
        self.client
            .set_activity(
                Activity::new()
                    .state(state.as_str())
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
}
