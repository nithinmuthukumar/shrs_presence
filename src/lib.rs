mod builtin;
use std::time::{SystemTime, UNIX_EPOCH};

use builtin::PresenceBuiltin;
use discord_rich_presence::{
    activity::{self, Activity, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use shrs::{
    anyhow::{self, anyhow, Result},
    prelude::*,
};
pub struct PresenceState {
    pub client: DiscordIpcClient,
    pub start: i64,
    pub commands_used: i32,
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
        }
    }
    pub fn connect(&mut self) -> Result<()> {
        if !self.client.connect().is_ok() {
            return Err(anyhow!("Client did not connect"));
        }
        Ok(())
    }
    pub fn update_activity(&mut self) {
        self.client
            .set_activity(
                Activity::new()
                    .state("1 session(s) open")
                    .details(format!("{} commands used", self.commands_used).as_str())
                    .timestamps(Timestamps::new().start(self.start)),
            )
            .unwrap();
    }
    pub fn clear_activity(&mut self) {
        self.client.clear_activity();
    }
}

pub struct PresencePlugin;

impl Plugin for PresencePlugin {
    fn init(&self, shell: &mut shrs::ShellConfig) -> anyhow::Result<()> {
        let mut state = PresenceState::new();
        state.connect()?;
        state.update_activity();

        shell.builtins.insert("presence", PresenceBuiltin {});
        shell.state.insert(state);
        shell.hooks.insert(set_activity_hook);

        Ok(())
    }
}
//Start timer, count number of commands, display mode, show shell repo
//Current directory, number of shrs sessions (tracked in some global manner)
fn set_activity_hook(
    _sh: &Shell,
    sh_ctx: &mut Context,
    _sh_rt: &mut Runtime,
    ctx: &AfterCommandCtx,
) -> anyhow::Result<()> {
    if let Some(state) = sh_ctx.state.get_mut::<PresenceState>() {
        state.commands_used += 1;
        sh_ctx.out.println(state.commands_used)?;

        state.update_activity();
    }

    Ok(())
}

//
