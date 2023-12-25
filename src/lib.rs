use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use shrs::{
    anyhow::{self, anyhow},
    prelude::Plugin,
};

pub struct PresencePlugin;

impl Plugin for PresencePlugin {
    fn init(&self, shell: &mut shrs::ShellConfig) -> anyhow::Result<()> {
        if let Ok(mut client) = DiscordIpcClient::new("1188721913586003988") {
            if !client.connect().is_ok() {
                return Err(anyhow!("Client did not connect"));
            }
        }
        Ok(())
    }
}
//Start timer, count number of commands, display mode, show shell repo
//Current directory, number of shrs sessions (tracked in some global manner)
