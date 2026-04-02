use clap::{ArgMatches, Command};

mod commands;

pub fn build_cli() -> Command {
    Command::new("image-to-ascii")
        .about("A CLI tool to generate ASCII arts from a input image.")
        .subcommand(commands::generate::command())
}

pub fn handle_matches(matches: ArgMatches) -> anyhow::Result<()> {
    match matches.subcommand() {
        Some(("generate", sub_m)) => commands::generate::run(sub_m)?,
        _ => println!("No subcommand was used"),
    }

    Ok(())
}
