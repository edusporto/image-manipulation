extern crate image;
use image::{ImageBuffer, Rgb};

use std::error::Error;

const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;
type Color = Rgb<u8>;

pub fn calc_location(size_pattern: (u32, u32), location: (u32, u32)) -> (u32, u32) {
    if location.0 < size_pattern.0 && location.1 < size_pattern.1 {
        return location;
    }

    (
        location.0 - size_pattern.0 * (location.0 / size_pattern.0),
        location.1 - size_pattern.1 * (location.1 / size_pattern.1),
    )
}

fn lighten(color: Color) -> Color {
    Rgb([
        add(color[0], 255 - color[0]),
        add(color[1], 255 - color[1]),
        add(color[1], 255 - color[2]),
    ])
}

pub fn draw_pattern(pixel: &mut (u32, u32, &mut Color), pattern: &Image, color: Color) {
    let loc = calc_location((pattern.width(), pattern.height()), (pixel.0, pixel.1));
    *pixel.2 = if *pattern.get_pixel(loc.0, loc.1) == BLACK {
        color
    } else {
        lighten(color)
    }
}

fn sub(lhs: u8, rhs: u8) -> u8 {
    lhs.checked_sub(rhs).unwrap_or(0)
}

fn add(lhs: u8, rhs: u8) -> u8 {
    lhs.checked_add(rhs).unwrap_or(255)
}

fn between(smaller: u8, val: u8, bigger: u8) -> bool {
    smaller <= val && val <= bigger
}

pub fn color_in_range(query: Color, color: Color, threshold: u8) -> bool {
    between(sub(query[0], threshold), color[0], add(query[0], threshold))
        && between(sub(query[1], threshold), color[1], add(query[1], threshold))
        && between(sub(query[2], threshold), color[2], add(query[2], threshold))
}

pub fn read_patterns(dir: &str) -> Result<Vec<Image>, Box<dyn Error>> {
    let mut patterns = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            patterns.push(image::open(path)?.to_rgb());
        }
    }

    Ok(patterns)
}

use std::env::Args;

pub fn read_colors(args: &mut Args, img: &Image) -> Result<Vec<(Color, u8)>, Box<dyn Error>> {
    let mut colors = Vec::new();

    for coords in args {
        let mut coords = coords.split(' ');

        let x: u32 = coords
            .next()
            .ok_or("Could not find x coordinate for a color")?
            .parse()?;

        let y: u32 = coords
            .next()
            .ok_or("Could not find y coordinate for a color")?
            .parse()?;

        let threshold: u8 = coords
            .next()
            .ok_or("Could not read threshold value for a color")?
            .parse()?;

        colors.push((*img.get_pixel(x, y), threshold));
    }

    Ok(colors)
}
