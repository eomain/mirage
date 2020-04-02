//! # Raster
//! Convert vector graphics into raster graphics

extern crate image;

mod object;

use crate::surface::{
    Object,
    Surface
};

use image::ColorType;
pub use image::error::ImageResult;

/// Data for a single picture element of a raster image
#[derive(Debug, Copy, Clone, PartialEq)]
enum Pixel {
    /// Data for red, green and blue channels
    Rgb(u8, u8, u8),
    /// Data for red, green, blue and alpha channels
    Rgba(u8, u8, u8, u8)
}

/// The type of a pixel
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PixelType {
    /// Red, green and blue channels
    Rgb,
    /// Red, green, blue and alpha channels
    Rgba
}

impl PixelType {

    /// The number of channels in a pixel
    pub fn channels(&self) -> usize
    {
        use PixelType::*;
        match self {
            Rgb => 3,
            Rgba => 4
        }
    }

    /// A pixel matches the type
    fn matches(&self, p: &Pixel) -> bool
    {
        use PixelType::*;
        match self {
            Rgb => match p {
                Pixel::Rgb(_, _, _) => true,
                _ => false
            },
            Rgba => match p {
                Pixel::Rgba(_, _, _, _) => true,
                _ => false
            }
        }
    }
}

impl From<PixelType> for ColorType {
    fn from(p: PixelType) -> Self
    {
        use PixelType::*;
        match p {
            Rgb => ColorType::Rgb8,
            Rgba => ColorType::Rgba8
        }
    }
}

/// An error from trying to create a raster image
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    /// Object position out of the
    /// bounds of the image
    Bound,
    /// Wrong pixel type
    Pixel
}

mod color {
    pub const WHITE: u8 = 0xFF;
}

/// A raster image
pub struct Image {
    /// Image filename
    pub name: String,
    /// Width of the image
    pub width: usize,
    /// Height of the image
    pub height: usize,
    /// The type of pixels
    pixel: PixelType,
    /// Image pixel buffer
    buffer: Vec<u8>
}

impl Image {
    /// Create a new blank image
    pub fn new<I>(name: I, width: usize, height: usize, pixel: PixelType) -> Self
        where I: Into<String>
    {
        Self {
            name: name.into(),
            width, height,
            pixel,
            buffer: vec![color::WHITE; pixel.channels() * (width * height)]
        }
    }

    #[inline]
    fn index(&self, pos: (usize, usize)) -> usize
    {
        let count = self.pixel.channels();
        let x = pos.0;
        let y = (pos.1 * (self.width));
        count * (x + y)
    }

    // TODO: refactor
    fn pixel(&mut self, pixel: Pixel, pos: (usize, usize)) -> Result<(), Error>
    {
        self.pixels(&[pixel], pos)
    }

    fn pixels(&mut self, pixels: &[Pixel], pos: (usize, usize)) -> Result<(), Error>
    {
        for p in pixels {
            if !self.pixel.matches(&p) {
                return Err(Error::Pixel);
            }
        }

        if pos.0 < self.width && pos.1 < self.height {
            use Pixel::*;

            let count = self.pixel.channels();
            let mut index = self.index(pos);
            let bytes = &mut self.buffer;

            for p in pixels {
                match *p {
                    Rgb(r, g, b) => {
                        bytes[index] = r;
                        bytes[index + 1] = g;
                        bytes[index + 2] = b;
                    },
                    Rgba(r, g, b, a) => {
                        bytes[index] = r;
                        bytes[index + 1] = g;
                        bytes[index + 2] = b;
                        bytes[index + 3] = a;
                    }
                }
                index += count;
            }
            Ok(())
        } else {
            Err(Error::Bound)
        }
    }

    /// Write a surface to the image
    pub fn write(&mut self, s: &Surface)
    {
        use PixelType::*;
        // TODO:
        let pixel = match self.pixel {
            Rgb => Pixel::Rgb(0, 0, 0),
            Rgba => Pixel::Rgba(0, 0, 0, 0)
        };

        s.for_each_mut(|o| {
            let pos = object::object(o);
            for p in pos {
                self.pixel(pixel, p);
            }
        });
    }

    /// Save the image
    pub fn save(&self) -> ImageResult<()>
    {
        image::save_buffer(
            &self.name,
            &self.buffer,
            self.width as u32,
            self.height as u32,
            self.pixel.into()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::convert::svg;

    #[test]
    fn image_test()
    {
        let svg = r#"
            <svg>
                <line x1="4" y1="4" x2="5" y2="5" />
                <line x1="8" y1="20" x2="9" y2="21" />
                <line x1="20" y1="20" x2="20" y2="80" />
                <line x1="8" y1="70" x2="60" y2="70" />
                <line x1="15" y1="20" x2="60" y2="75" />
                <rect x="60" y="30" width="30" height="20" />
            </svg>
        "#;
        let surface = svg::into::string(svg).unwrap();

        let mut image = Image::new("pic.png", 100, 100, PixelType::Rgb);
        image.write(&surface);
        image.save();
    }
}
