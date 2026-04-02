const CHARS: &str = " .:-=+*#%@";

pub fn calculate_luminosity(r: u8, g: u8, b: u8) -> f64 {
    let scale: f64 = 0.299 * (r as f64) + 0.587 * (g as f64) + 0.114 * (b as f64);

    scale
}

pub fn map_to_char(scale: f64) -> anyhow::Result<char> {
    let gamma: f64 = 2.2;
    let contrast: f64 = 1.2;

    let normalized: f64 = scale / 255.0;
    let corrected_scale: f64 = normalized.powf(1.0 / gamma);
    let contrasted: f64 = ((corrected_scale - 0.5) * contrast + 0.5).clamp(0.0, 1.0);

    let index: usize = (contrasted * (CHARS.len() - 1) as f64) as usize;

    let character: Option<char> = CHARS.chars().nth(index);
    if character.is_none() {
        anyhow::bail!("Invalid scale value: {}", contrasted);
    }

    Ok(character.unwrap())
}
