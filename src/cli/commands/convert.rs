use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command {
    Command::new("convert")
        .about("Converts an input image into a output ASCII version")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("Input image path"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output folder path"),
        )
}

pub fn run(matches: &ArgMatches) -> anyhow::Result<()> {
    let raw_input: Option<&String> = matches.get_one::<String>("input");
    let raw_output: Option<&String> = matches.get_one::<String>("output");

    if raw_input.is_none() {
        anyhow::bail!("Input image argument is required");
    }
    if raw_output.is_none() {
        anyhow::bail!("Output path argument is required");
    }

    let input: &String = raw_input.unwrap();
    let output: &String = raw_output.unwrap();

    println!("Attempt to convert\nInput: {}\nOutput: {}", input, output);
    Ok(())
}
