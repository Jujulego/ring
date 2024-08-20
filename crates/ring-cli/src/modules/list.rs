use clap::Command;
use ring_core::RingCore;

pub fn build_command() -> Command {
    Command::new("list")
        .visible_alias("ls")
}

pub fn handle_command(core: &RingCore) -> anyhow::Result<()> {
    for module in core.modules() {
        println!("{}", module.name());
    }
    
    Ok(())
}