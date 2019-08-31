extern crate image;

use image_manipulation::*;
use std::error::Error;

// TODO: test on Android

fn main() -> Result<(), Box<dyn Error>> {
    let dyn_img = image::open("image.png")?;
    let img = dyn_img.to_rgb();
    let mut output = img.clone();
    // let img_luma = dyn_img.to_luma();

    let patterns = vec![
        image::open("patterns/0.png")?.to_rgb(),
        image::open("patterns/1.png")?.to_rgb(),
        image::open("patterns/2.png")?.to_rgb(),
    ];

    let colors = vec![
        image::Rgb([209, 255, 125]),
        image::Rgb([149, 190, 0]),
        image::Rgb([158, 20, 43]),
    ];

    // Processing

    for pixel in img.enumerate_pixels() {
        for (i, color) in colors.iter().enumerate() {
            if color_in_range(*pixel.2, *color, 1) {
                draw_pattern(
                    &mut (pixel.0, pixel.1, output.get_pixel_mut(pixel.0, pixel.1)),
                    &patterns[i],
                    *color,
                );
            }
        }
    }

    output.save("output.png")?;

    Ok(())
}
