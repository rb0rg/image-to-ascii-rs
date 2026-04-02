use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn write(ascii: &Vec<char>, path: &PathBuf) -> anyhow::Result<()> {
    let mut file = File::create(path)?;

    file.write_all(ascii.iter().collect::<String>().as_bytes())?;

    Ok(())
}
