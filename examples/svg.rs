extern crate mirage;

use mirage::convert::svg;
use mirage::raster::{
    Image,
    PixelType
};

static SVG: &str = include_str!("rect.svg");

fn main()
{
    let surface = svg::into::string(SVG).unwrap();
    let mut image = Image::new("pic.png", 100, 100, PixelType::Rgb);
    image.write(&surface);
    image.save();
}
