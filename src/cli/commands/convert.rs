use std::path::PathBuf;

use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command {
    Command::new("convert")
        .about("Converts an input image into a output ASCII version")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .required(true)
                .value_parser(clap::value_parser!(PathBuf))
                .help("Input image path"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .required(true)
                .value_parser(clap::value_parser!(PathBuf))
                .help("Output folder path"),
        )
}

pub fn run(matches: &ArgMatches) -> anyhow::Result<()> {
    let input: &PathBuf = matches.get_one("input").unwrap();
    let output: &PathBuf = matches.get_one("output").unwrap();

    if !input.exists() {
        anyhow::bail!("Input image path does not exist");
    }
    if !output.exists() {
        anyhow::bail!("Output path does not exist");
    }

    println!(
        "Attempt to convert\nInput: {:?}\nOutput: {:?}",
        input, output
    );
    Ok(())
}
