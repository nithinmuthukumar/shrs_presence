mod builtin;
use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use builtin::PresenceBuiltin;
use discord_rich_presence::{
    activity::{self, Activity, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use shrs::{
    anyhow::{self, anyhow, Result},
    prelude::*,
};
//OPTIONS
//Start timer, count number of commands, display mode, show shell repo
//Current directory, number of shrs sessions (tracked in some global manner)

pub struct PresenceState {
    pub client: DiscordIpcClient,
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
        }
    }
    pub fn connect(&mut self) -> Result<()> {
        if !self.client.connect().is_ok() {
            return Err(anyhow!("Client did not connect"));
        }
        Ok(())
    }

    pub fn update_activity(&mut self, state: &str) {
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

        shell.builtins.insert("presence", PresenceBuiltin {});
        shell.state.insert(state);
        shell.hooks.insert(startup_hook);
        shell.hooks.insert(begin_hook);
        shell.hooks.insert(after_hook);

        Ok(())
    }
}
fn after_hook(
    _sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &AfterCommandCtx,
) -> anyhow::Result<()> {
    if let Some(state) = sh_ctx.state.get_mut::<PresenceState>() {
        state.commands_used += 1;
        state.working_dir = sh_rt.working_dir.clone();

        state.update_activity(
            format!("Completed \"{}\" ({})", ctx.command, ctx.cmd_output.status).as_str(),
        );
    }

    Ok(())
}

fn begin_hook(
    _sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &BeforeCommandCtx,
) -> anyhow::Result<()> {
    if let Some(state) = sh_ctx.state.get_mut::<PresenceState>() {
        state.commands_used += 1;
        state.working_dir = sh_rt.working_dir.clone();

        state.update_activity(format!("Running \"{}\"", ctx.command).as_str());
    }

    Ok(())
}
fn startup_hook(
    _sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &StartupCtx,
) -> anyhow::Result<()> {
    if let Some(state) = sh_ctx.state.get_mut::<PresenceState>() {
        state.commands_used += 1;
        state.working_dir = sh_rt.working_dir.clone();
        state.update_activity("Starting Shell");
    }

    Ok(())
}

//
