mod builtin;
pub mod state;
use std::{
    os::unix::process::ExitStatusExt,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use builtin::PresenceBuiltin;
use discord_rich_presence::{
    activity::{self, Activity, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use shrs::{
    anyhow::{self, anyhow, Result},
    prelude::*,
};
use state::PresenceState;

pub struct PresencePlugin {
    shell_repo: String,
}
impl PresencePlugin {
    pub fn new(shell_repo: String) -> Self {
        Self { shell_repo }
    }
}
impl Default for PresencePlugin {
    fn default() -> Self {
        Self::new("https://github.com/MrPicklePinosaur/shrs".to_string())
    }
}

impl Plugin for PresencePlugin {
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {
        let mut state = PresenceState::new();
        state.connect();

        shell.builtins.insert("presence", PresenceBuiltin {});
        shell.state.insert(state);
        shell.hooks.insert(startup_hook);
        shell.hooks.insert(update_activity_hook);
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

        if !ctx.command.is_empty() {
            state.update_activity(
                format!(
                    "> \"{}\" (code: {})",
                    ctx.command,
                    ctx.cmd_output.status.into_raw()
                )
                .as_str(),
            );
        }
    }

    Ok(())
}

fn update_activity_hook(
    _sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &BeforeCommandCtx,
) -> anyhow::Result<()> {
    if let Some(state) = sh_ctx.state.get_mut::<PresenceState>() {
        state.commands_used += 1;
        state.working_dir = sh_rt.working_dir.clone();
        if !ctx.command.is_empty() {
            state.update_activity(format!("> \"{}\"", ctx.command).as_str());
        }
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
