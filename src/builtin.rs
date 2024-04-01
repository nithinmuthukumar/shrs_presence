use clap::{Parser, Subcommand};
use shrs::{
    anyhow::Result,
    prelude::{BuiltinCmd, CmdOutput},
};

use crate::PresenceState;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Debug, Subcommand)]
enum Commands {}
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
        if let Some(state) = ctx.state.get_mut::<PresenceState>() {
            if let Some(c) = cli.command {
            } else {
                match reqwest::blocking::get(state.url.clone() + "/info") {
                    Ok(res) => ctx.out.println(res.text()?)?,
                    Err(_) => ctx.out.println("No server running")?,
                }
            }
        }
        Ok(CmdOutput::success())
    }
}
