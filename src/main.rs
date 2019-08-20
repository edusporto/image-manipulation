extern crate image;

use std::error::Error;

fn calc_location(
    size2: (u32, u32),
    location: (u32, u32),
) -> (u32, u32) {
    
    if location.0 < size2.0 && location.1 < size2.1 {
        return location;
    }

    let mut ret = (0, 0);

    let a = location.0 / size2.0;
    let b = location.1 / size2.1;

    ret.0 = location.0 - size2.0 * a;
    ret.1 = location.1 - size2.1 * b;

    ret
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut img = image::open("image.png").unwrap().to_rgb();
<<<<<<< HEAD
    let pattern = image::open("pattern.jpg").unwrap().to_rgb();

    let old_color = image::Rgb([209, 255, 125]);
    // let new_color = image::Rgb([20, 20, 240]);
    //let mut pixels_to_change = Vec::<image::Rgb<u8>>::new();
=======
    
    let gray = image::Rgb([128_u8, 128, 128]);
    
    // TODO: use rayon
    // https://rust-lang-nursery.github.io/rust-cookbook/concurrency/parallel.html
    // https://stackoverflow.com/questions/48922420/how-do-i-use-rayon-with-an-existing-iterator/54201274#54201274

    for pixel in img.pixels_mut() {
        let rgb = pixel.to_rgb();
    
        if rgb[0] > 240 {
            if rgb[1] > 240 {
                if rgb[2] > 240 {
                    *pixel = gray;
                }
            }
        }
>>>>>>> a56a0e3a9236d91d76c902b2ebd1f6bd51628b83

    let size1 = (img.width(), img.height());
    let size2 = (pattern.width(), pattern.height());

    for p in img.enumerate_pixels_mut() {
        if *p.2 == old_color {
            //pixels_to_change.push(*p);

            let loc = calc_location(size2, (p.0, p.1));
            *p.2 = *pattern.get_pixel(loc.0, loc.1);
        }
    }

    img.save("output.png")?;

    Ok(())
}
