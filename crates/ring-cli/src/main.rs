use anyhow::Result;
use clap::command;

mod js;

fn main() -> Result<()> {
    let args = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .subcommand(js::build_command())
        .get_matches();
    
    match args.subcommand() {
        Some(("js", args)) => js::handle_command(args),
        _ => unreachable!()
    }
}
