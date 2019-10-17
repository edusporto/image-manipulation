extern crate image;

use image_manipulation::*;

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();

    let dyn_img = image::open(args.next().ok_or("Please specify the image to process")?)?;
    let img = dyn_img.to_rgb();
    let mut output = img.clone();

    let patterns = read_patterns("patterns")?;
    let colors = read_colors(&mut args, &img)?;

    // Processing

    for pixel in img.enumerate_pixels() {
        for (i, color) in colors.iter().enumerate() {
            if color_in_range(*pixel.2, color.0, color.1) {
                draw_pattern(
                    &mut (pixel.0, pixel.1, output.get_pixel_mut(pixel.0, pixel.1)),
                    &patterns[i],
                    color.0,
                );
            }
        }
    }

    output.save("output.png")?;

    Ok(())
}
