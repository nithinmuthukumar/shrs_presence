mod builtin;

use std::{
    env::home_dir,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    time::Duration,
};

use builtin::PresenceBuiltin;
use reqwest::Client;
use serde_json::json;
use shrs::{
    anyhow::{anyhow, Ok, Result},
    prelude::*,
};
use std::process::Command;
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
                let _ = client.post(u.clone() + "/connect").json(&data).send().await;
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
fn setup_presence_daemon(config_dir: &PathBuf) -> Result<()> {
    let cargo_dir = dirs::home_dir()
        .ok_or(anyhow!("No Home Dir"))?
        .join(".cargo/bin/shrs_presence_server");
    let plist_file = config_dir.join("presence/com.nithin.shrs_presence_server.plist");
    if let Some(parent) = plist_file.parent() {
        fs::create_dir_all(parent)?;
    }
    let s = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
   <dict>
       <key>Label</key>
       <string>com.nithin.shrs_presence_server</string>
       <key>Program</key>
       <string>{}</string>
       <key>RunAtLoad</key>
       <true/>
       <key>StandardOutPath</key>
       <string>{}/presence/logfile.log</string>
       <key>StandardErrorPath</key>
       <string>{}/presence/error.log</string>
   </dict>
</plist>"#,
        cargo_dir.to_string_lossy(),
        config_dir.to_string_lossy(),
        config_dir.to_string_lossy()
    );
    File::create(plist_file.clone())?.write_all(s.as_bytes())?;
    let output = Command::new("launchctl")
        .arg("load")
        .arg(plist_file)
        .output()?;

    // Check if the job started successfully
    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow!("launchctl failed"))
    }
}

impl Plugin for PresencePlugin {
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {
        setup_presence_daemon(&shell.config_dir)?;
        shell.builtins.insert("presence", PresenceBuiltin {});
        shell
            .state
            .insert(PresenceState::new("http://127.0.0.1:3000".to_string()));
        shell.hooks.insert(startup_hook);
        shell.hooks.insert(command_hook);

        Ok(())
    }

    fn meta(&self) -> PluginMeta {
        PluginMeta::new(
            "Presence",
            "Plugin to enable discord presence",
            Some("Please make sure the server is installed"),
        )
    }

    fn fail_mode(&self) -> FailMode {
        // Default to more strict fail mode to let users know faster there's a bug
        //
        // Should consider more how good of an idea this is
        FailMode::Abort
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
