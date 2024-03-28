mod builtin;

use builtin::PresenceBuiltin;
use serde_json::json;
use shrs::{anyhow::Result, prelude::*};
use uuid::Uuid;
pub struct PresenceState {
    id: Uuid,
}
impl PresenceState {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}
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
        shell.builtins.insert("presence", PresenceBuiltin {});
        shell.state.insert(PresenceState::new());
        shell.hooks.insert(startup_hook);
        shell.hooks.insert(command_hook);

        Ok(())
    }
}
fn command_hook(
    _sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &BeforeCommandCtx,
) -> Result<()> {
    let client = reqwest::blocking::Client::new();
    if let Some(state) = sh_ctx.state.get::<PresenceState>() {
        let data = json!({ "id":state.id.to_string() });
        let res = client
            .post("http://127.0.0.1:3000/command/add")
            .json(&data)
            .send()?;
    }
    Ok(())
}

fn startup_hook(
    _sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &StartupCtx,
) -> Result<()> {
    let client = reqwest::blocking::Client::new();
    if let Some(state) = sh_ctx.state.get::<PresenceState>() {
        let data = json!({ "id":state.id.to_string() });
        let res = client
            .post("http://127.0.0.1:3000/connect")
            .json(&data)
            .send()?;
    }

    Ok(())
}

//
