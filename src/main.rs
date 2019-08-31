extern crate image;
use image::{ImageBuffer, Rgb};

use std::error::Error;

// TODO: check polyfloyd/edge-detection-rs repository on GitHub
// TODO: test on Android

fn calc_location(size_pattern: (u32, u32), location: (u32, u32)) -> (u32, u32) {
    if location.0 < size_pattern.0 && location.1 < size_pattern.1 {
        return location;
    }

    (
        location.0 - size_pattern.0 * (location.0 / size_pattern.0),
        location.1 - size_pattern.1 * (location.1 / size_pattern.1),
    )
}

fn draw_pattern(
    pixel: &mut (u32, u32, &mut image::Rgb<u8>),
    pattern: &ImageBuffer<Rgb<u8>, Vec<u8>>,
) {
    let loc = calc_location((pattern.width(), pattern.height()), (pixel.0, pixel.1));
    *pixel.2 = *pattern.get_pixel(loc.0, loc.1);
}

fn main() -> Result<(), Box<dyn Error>> {
    let dyn_img = image::open("image.png")?;
    let mut img = dyn_img.to_rgb();
    let img_luma = dyn_img.to_luma();

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

    let edges = imageproc::edges::canny(&img_luma, 10.0, 50.0);
    //let color_edges = imageproc::map::map_colors(&edges, |p| if p[0] > 0 { white } else { black });

    edges.save("edges.png")?;

    for mut pixel in img.enumerate_pixels_mut() {
        for (i, color) in colors.iter().enumerate() {
            if pixel.2 == color {
                draw_pattern(&mut pixel, &patterns[i]);
            }
        }
    }

    img.save("output.png")?;

    Ok(())
}
