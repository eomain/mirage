
use super::Point;
use super::Translate;

/// Standard pixel size
pub const SIZE: u32 = 10;

/// A simple text object
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Text {
    /// Position of the text
    pub point: Point,
    /// The actual text string
    pub text: String,
    /// The font size,
    pub size: u32
}

impl Text {
    pub fn new<P, I>(point: P, text: I) -> Self
        where P: Into<Point>, I: Into<String>
    {
        Self {
            point: point.into(),
            text: text.into(),
            size: SIZE
        }
    }
}

impl Translate for Text {
    fn point(&self) -> &Point
    {
        &self.point
    }

    fn point_mut(&mut self) -> &mut Point
    {
        &mut self.point
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_variables)]
    use super::*;

    #[test]
    fn text()
    {
        let t = Text::new((1, 1), "hello world");
    }
}
