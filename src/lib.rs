extern crate image;
use image::{ImageBuffer, Rgb};

use std::error::Error;

const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;
type Color = Rgb<u8>;

#[derive(Debug, Copy, Clone)]
pub struct ColorThreshold {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

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

fn darken(color: Color) -> Color {
    Rgb([color[0] / 4, color[1] / 4, color[2] / 4])
}

fn is_light(color: Color) -> bool {
    color[0] > 204 && color[1] > 204 && color[2] > 204
}

pub fn draw_pattern(pixel: &mut (u32, u32, &mut Color), pattern: &Image, color: Color) {
    let loc = calc_location((pattern.width(), pattern.height()), (pixel.0, pixel.1));
    *pixel.2 = if *pattern.get_pixel(loc.0, loc.1) == BLACK {
        color
    } else if is_light(color) {
        darken(color)
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

pub fn color_in_range(query: Color, color: Color, threshold: ColorThreshold) -> bool {
    between(
        sub(query[0], threshold.red),
        color[0],
        add(query[0], threshold.red),
    ) && between(
        sub(query[1], threshold.green),
        color[1],
        add(query[1], threshold.green),
    ) && between(
        sub(query[2], threshold.blue),
        color[2],
        add(query[2], threshold.blue),
    )
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

#[derive(Debug)]
pub struct ColorPickingError(&'static str);

impl std::fmt::Display for ColorPickingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Error when choosing which colors to change to patterns: {}",
            self.0
        )
    }
}

impl std::convert::From<std::num::ParseIntError> for ColorPickingError {
    fn from(_: std::num::ParseIntError) -> Self {
        ColorPickingError("Failed to parse an int value")
    }
}

impl Error for ColorPickingError {}

pub fn read_colors(
    args: &mut Args,
    img: &Image,
) -> Result<Vec<(Color, ColorThreshold)>, ColorPickingError> {
    let mut colors = Vec::new();

    for color_options in args {
        let color_options: Vec<&str> = color_options.split(' ').collect();

        if color_options.len() < 2 {
            return Result::Err(ColorPickingError("Missing color coordinates"));
        }

        if color_options.len() < 5 {
            return Result::Err(ColorPickingError("Missing color threshold values"));
        }

        let x: u32 = color_options[0].parse()?;
        let y: u32 = color_options[1].parse()?;
        let red: u8 = color_options[2].parse()?;
        let green: u8 = color_options[3].parse()?;
        let blue: u8 = color_options[4].parse()?;

        colors.push((*img.get_pixel(x, y), ColorThreshold { red, green, blue }));
    }

    Ok(colors)
}
