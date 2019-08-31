extern crate image;
use image::{ImageBuffer, Rgb};

const BLACK: Rgb<u8> = Rgb([0, 0, 0]);

pub fn calc_location(size_pattern: (u32, u32), location: (u32, u32)) -> (u32, u32) {
    if location.0 < size_pattern.0 && location.1 < size_pattern.1 {
        return location;
    }

    (
        location.0 - size_pattern.0 * (location.0 / size_pattern.0),
        location.1 - size_pattern.1 * (location.1 / size_pattern.1),
    )
}

fn lighten(color: Rgb<u8>) -> Rgb<u8> {
    Rgb([
        add(color[0], 255 - color[0]),
        add(color[1], 255 - color[1]),
        add(color[1], 255 - color[2]),
    ])
}

pub fn draw_pattern(
    pixel: &mut (u32, u32, &mut image::Rgb<u8>),
    pattern: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    color: Rgb<u8>,
) {
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

pub fn color_in_range(query: Rgb<u8>, color: Rgb<u8>, threshold: u8) -> bool {
    between(sub(query[0], threshold), color[0], add(query[0], threshold))
        && between(sub(query[1], threshold), color[1], add(query[1], threshold))
        && between(sub(query[2], threshold), color[2], add(query[2], threshold))
}
