use shrs::ShellBuilder;
use shrs_presence::PresencePlugin;

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(PresencePlugin)
        .build()
        .unwrap();
    myshell.run();
}
