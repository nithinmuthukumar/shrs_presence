mod builtin;

use std::time::Duration;

use builtin::PresenceBuiltin;
use reqwest::Client;
use serde_json::json;
use shrs::{anyhow::Result, prelude::*};
use uuid::Uuid;
pub struct PresenceState {
    url: String,
    id: Uuid,
    _rt: tokio::runtime::Runtime,
}
impl PresenceState {
    pub fn new(url: String) -> Self {
        let id = Uuid::new_v4();
        let u = url.clone();

        let tokio_rt = tokio::runtime::Runtime::new().unwrap();
        tokio_rt.spawn(async move {
            let data = json!({ "id":id.to_string() });

            let client = Client::new();
            loop {
                tokio::time::sleep(Duration::from_secs(20)).await;
                client.post(u.clone() + "/connect").json(&data).send().await;
            }
        });

        Self {
            id,
            _rt: tokio_rt,
            url,
        }
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
        shell
            .state
            .insert(PresenceState::new("http://127.0.0.1:3000".to_string()));
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
        let _ = client
            .post(state.url.clone() + "/command/add")
            .body(ctx.command.clone())
            .send();
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
        let _ = client
            .post(state.url.clone() + "/connect")
            .json(&data)
            .send();
    }

    Ok(())
}

//
