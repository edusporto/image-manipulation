extern crate image;
extern crate rayon;

use rayon::prelude::*;

use image::{Pixel};

fn main() {
    let mut img = image::open("image.png").unwrap().to_rgb();
    
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

        /*if pixel.to_rgb() == white {
            *pixel = gray;
        }*/
    }

    img.save("output.png");
}
