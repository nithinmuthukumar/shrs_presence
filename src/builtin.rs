use shrs::{anyhow::Result, prelude::BuiltinCmd};

pub struct PresenceBuiltin {}
impl BuiltinCmd for PresenceBuiltin {
    fn run(
        &self,
        sh: &shrs::prelude::Shell,
        ctx: &mut shrs::prelude::Context,
        rt: &mut shrs::prelude::Runtime,
        args: &[String],
    ) -> Result<shrs::prelude::CmdOutput> {
        todo!()
    }
}
