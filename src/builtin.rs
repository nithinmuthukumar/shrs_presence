use clap::{Parser, Subcommand};
use shrs::{
    anyhow::Result,
    prelude::{BuiltinCmd, CmdOutput},
};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Debug, Subcommand)]
enum Commands {
    Connect,
    Disconnect,
}
pub struct PresenceBuiltin {}
impl BuiltinCmd for PresenceBuiltin {
    fn run(
        &self,
        sh: &shrs::prelude::Shell,
        ctx: &mut shrs::prelude::Context,
        rt: &mut shrs::prelude::Runtime,
        args: &[String],
    ) -> Result<shrs::prelude::CmdOutput> {
        let cli = Cli::try_parse_from(args)?;
        if let Some(c) = cli.command {
            match c {
                Commands::Connect => {
                    let success = true;
                    if !success {
                        ctx.out.eprintln("Could not connect to discord")?;
                        return Ok(CmdOutput::error());
                    } else {
                        ctx.out.println("Connected")?;
                    }
                }
                Commands::Disconnect => {
                    // state.disconnect();
                    ctx.out.println("Disconnected")?;
                }
            }
        } else {
            // ctx.out.println(format!(
            //     "client_id:{}\nconnected:{}",
            //     state.client.client_id, state.connected
            // ))?;
        }
        Ok(CmdOutput::success())
    }
}
