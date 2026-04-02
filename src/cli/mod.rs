use clap::{ArgMatches, Command};

mod commands;

pub fn build_cli() -> Command {
    Command::new("image-to-ascii")
        .about("")
        .subcommand(commands::generate::command())
}

pub fn handle_matches(matches: ArgMatches) -> anyhow::Result<()> {
    match matches.subcommand() {
        Some(("generate", sub_m)) => commands::generate::run(sub_m)?,
        _ => println!("No subcommand was used"),
    }

    Ok(())
}
