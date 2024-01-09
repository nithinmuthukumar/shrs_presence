use shrs::ShellBuilder;
use shrs_presence::PresencePlugin;

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(PresencePlugin::default())
        .build()
        .unwrap();
    myshell.run();
}
