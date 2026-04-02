use std::path::PathBuf;

use image::GenericImageView;
use image::ImageReader;

mod gray_scale;

#[allow(unused_must_use)]
pub fn gen_from_image(path: &PathBuf) -> anyhow::Result<Vec<char>> {
    let mut vec = Vec::new();

    let mut img = ImageReader::open(path)?.decode()?;

    let max_width = 300.0;

    let (mut width, mut height) = img.dimensions();

    let aspect_ratio = height as f64 / width as f64;

    let char_ratio = 0.5;

    let new_width = max_width;
    let new_height = (aspect_ratio * new_width * char_ratio) as u32;

    img = img.resize(
        new_width as u32,
        new_height,
        image::imageops::FilterType::Triangle,
    );

    width = img.width();
    height = img.height();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let scale = gray_scale::calculate_luminosity(pixel[0], pixel[1], pixel[2]);
            let character = gray_scale::map_to_char(scale)?;

            vec.push(character);
        }

        vec.push('\n');
    }

    Ok(vec)
}
