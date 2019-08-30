extern crate image;

use std::error::Error;

fn calc_location(size_pattern: (u32, u32), location: (u32, u32)) -> (u32, u32) {
    if location.0 < size_pattern.0 && location.1 < size_pattern.1 {
        return location;
    }

    (
        location.0 - size_pattern.0 * (location.0 / size_pattern.0),
        location.1 - size_pattern.1 * (location.1 / size_pattern.1),
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let dyn_img = image::open("image.png")?;
    let mut img = dyn_img.to_rgb();
    let img_luma = dyn_img.to_luma();

    let pattern1 = image::open("patterns/0.png")?.to_rgb();
    let pattern2 = image::open("patterns/1.png")?.to_rgb();
    let pattern3 = image::open("patterns/2.png")?.to_rgb();

    let color1 = image::Rgb([209, 255, 125]);
    let color2 = image::Rgb([149, 190, 0]);
    let color3 = image::Rgb([158, 20, 43]);
    //let white = image::Rgb([255, 255, 255]);
    //let black = image::Rgb([0, 0, 0]);

    let size1 = (pattern1.width(), pattern1.height());
    let size2 = (pattern2.width(), pattern2.height());
    let size3 = (pattern3.width(), pattern3.height());

    for pixel in img.enumerate_pixels_mut() {
        if *pixel.2 == color1 {
            let loc = calc_location(size1, (pixel.0, pixel.1));
            *pixel.2 = *pattern1.get_pixel(loc.0, loc.1);
        }

        if *pixel.2 == color2 {
            let loc = calc_location(size2, (pixel.0, pixel.1));
            *pixel.2 = *pattern2.get_pixel(loc.0, loc.1);
        }

        if *pixel.2 == color3 {
            let loc = calc_location(size3, (pixel.0, pixel.1));
            *pixel.2 = *pattern3.get_pixel(loc.0, loc.1);
        }
    }

    let edges = imageproc::edges::canny(&img_luma, 50.0, 100.0);
    //let color_edges = imageproc::map::map_colors(&edges, |p| if p[0] > 0 { white } else { black });

    edges.save("lines.png")?;

    img.save("output.png")?;

    Ok(())
}
