mod cli;

fn main() -> anyhow::Result<()> {
    let cli = cli::build_cli();
    let matches = cli.get_matches();

    cli::handle_matches(matches)?;

    Ok(())
}
