use clap::{ArgMatches, Command};

mod commands;

pub fn build_cli() -> Command {
    Command::new("image-to-ascii")
        .about("")
        .subcommand(commands::convert::command())
}

pub fn handle_matches(matches: ArgMatches) -> anyhow::Result<()> {
    match matches.subcommand() {
        Some(("convert", sub_m)) => commands::convert::run(sub_m)?,
        _ => println!("No subcommand was used"),
    }

    Ok(())
}
