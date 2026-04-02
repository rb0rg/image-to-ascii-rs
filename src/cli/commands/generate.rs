use std::path::PathBuf;

use clap::{Arg, ArgMatches, Command};

use crate::core;
use crate::fs;

pub fn command() -> Command {
    Command::new("generate")
        .about("Generates an ASCII version of the input image")
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

    let spinner = indicatif::ProgressBar::new_spinner();

    spinner.set_message("Generating ASCII version...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let gen_ascii: Vec<char> = core::ascii::gen_from_image(input)?;

    spinner.set_message("Writing output...");

    let write_output = output
        .join(input.file_stem().unwrap())
        .with_extension("txt");

    fs::ascii_to_txt::write(&gen_ascii, &write_output)?;

    spinner.finish_with_message("Conversion complete!");

    Ok(())
}
