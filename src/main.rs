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
    let mut img = image::open("image.png")?.to_rgb();
    let pattern1 = image::open("pattern1.png")?.to_rgb();
    let pattern2 = image::open("pattern2.png")?.to_rgb();

    let color1 = image::Rgb([209, 255, 125]);
    let color2 = image::Rgb([149, 190, 0]);

    let size1 = (pattern1.width(), pattern1.height());
    let size2 = (pattern2.width(), pattern2.height());

    for p in img.enumerate_pixels_mut() {
        if *p.2 == color1 {
            //pixels_to_change.push(*p);

            let loc = calc_location(size1, (p.0, p.1));
            *p.2 = *pattern1.get_pixel(loc.0, loc.1);
        }

        if *p.2 == color2 {
            //pixels_to_change.push(*p);

            let loc = calc_location(size2, (p.0, p.1));
            *p.2 = *pattern2.get_pixel(loc.0, loc.1);
        }
    }

    img.save("output.png")?;

    Ok(())
}
