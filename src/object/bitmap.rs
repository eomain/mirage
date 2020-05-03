
use super::Point;
use super::Translate;

/// A bitmap image
#[derive(Debug, Clone, PartialEq)]
pub struct Bitmap {
    /// Position of the image
    pub point: Point,
    /// The image data
    pub data: Vec<u8>,
    /// Width of the image
    pub width: u32,
    /// Height of the image
    pub height: u32
}

impl Bitmap {
    /// Create a new bitmap object
    pub fn new<P, D>(point: P, data: D, width: u32, height: u32) -> Self
        where P: Into<Point>, D: Into<Vec<u8>>
    {
        Self {
            point: point.into(),
            data: data.into(),
            width,
            height
        }
    }
}

impl Translate for Bitmap {
    fn point(&self) -> &Point
    {
        &self.point
    }

    fn point_mut(&mut self) -> &mut Point
    {
        &mut self.point
    }
}
