//! Context for drawing vector graphics
//!
//! # Example usage
//! Use of context for simple drawing
//!
//! ```
//! use mirage::context::Context;
//!
//! // Create new context
//! let mut cx = Context::new();
//! // Set the color to `red`
//! cx.rgb(1.0, 0.0, 0.0);
//! // Draw rectangle
//! cx.rect((5, 5), 40, 25);
//! // Fill
//! cx.fill();
//! ```

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use crate::object::{Point, Rect};

#[derive(Debug, Clone, PartialEq)]
pub enum ImageType {
    Path(PathBuf),
    Data(RefCell<Vec<u8>>, ImageFormat, u32, u32)
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ImageFormat {
    /// Pixel is 24-bits, 8-bits per component
    Rgb8,
    /// Pixel is 32-bits, 8-bits per component
    Rgba8,
    /// Pixel is 24-bit, 8-bits per component
    Bgr8,
    /// Pixel is 32-bits, 8-bits per component
    Bgra8
}

/// Drawing command operations
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Rgb(f64, f64, f64),
    Rgba(f64, f64, f64, f64),
    Text(String),
    Image(Point, ImageType),
    FontSize(f64),
    Move(Point),
    RelMove(Point),
    Line(Point),
    RelLine(Point),
    Rect(Rect),
    RelRect(u32, u32),
    Arc(Point, u32, f64, f64),
    Curve(Point, Point, Point),
    Scale(f64, f64),
    Rotate(f64),
    Translate(f64, f64),
    Stroke,
    Fill,
    Paint
}

/// Context used for drawing operations
#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    commands: Vec<Command>
}

impl Context {
    pub fn new() -> Self {
        Self {
            commands: Vec::new()
        }
    }

    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    #[inline]
    pub fn rgb(&mut self, red: f64, green: f64, blue: f64) {
        self.commands.push(Command::Rgb(red, green, blue));
    }

    #[inline]
    pub fn rgba(&mut self, red: f64, green: f64, blue: f64, alpha: f64) {
        self.commands.push(Command::Rgba(red, green, blue, alpha));
    }

    #[inline]
    pub fn text<T>(&mut self, text: T)
        where T: AsRef<str> {
        self.commands.push(Command::Text(text.as_ref().into()));
    }

    #[inline]
    pub fn image<T, P>(&mut self, path: P, point: T)
        where T: Into<Point>, P: AsRef<Path> {
        self.commands.push(Command::Image(point.into(), ImageType::Path(path.as_ref().into())));
    }

    #[inline]
    pub fn image_data<D, P>(&mut self, data: D, format: ImageFormat, point: P,
        width: u32, height: u32)
        where D: AsRef<[u8]>, P: Into<Point> {
        let data = ImageType::Data(RefCell::new(data.as_ref().into()), format, width, height);
        self.commands.push(Command::Image(point.into(), data));
    }

    #[inline]
    pub fn font_size(&mut self, size: f64) {
        self.commands.push(Command::FontSize(size));
    }

    #[inline]
    pub fn move_to<P>(&mut self, point: P)
        where P: Into<Point> {
        self.commands.push(Command::Move(point.into()));
    }

    #[inline]
    pub fn rel_move_to<P>(&mut self, point: P)
        where P: Into<Point> {
        self.commands.push(Command::RelMove(point.into()));
    }

    #[inline]
    pub fn line_to<P>(&mut self, point: P)
        where P: Into<Point> {
        self.commands.push(Command::Line(point.into()));
    }

    #[inline]
    pub fn rel_line_to<P>(&mut self, point: P)
        where P: Into<Point> {
        self.commands.push(Command::RelLine(point.into()));
    }

    #[inline]
    pub fn rect<P>(&mut self, point: P, width: usize, height: usize)
        where P: Into<Point> {
        let rect = Rect::new(point, width, height);
        self.commands.push(Command::Rect(rect));
    }

    #[inline]
    pub fn rel_rect(&mut self, width: u32, height: u32) {
        self.commands.push(Command::RelRect(width, height));
    }

    #[inline]
    pub fn arc<P>(&mut self, point: P, radius: u32, angle1: f64, angle2: f64)
        where P: Into<Point> {
        self.commands.push(Command::Arc(point.into(), radius, angle1, angle2));
    }

    #[inline]
    pub fn curve_to<P>(&mut self, p1: P, p2: P, p3: P)
        where P: Into<Point> {
        self.commands.push(Command::Curve(p1.into(), p2.into(), p3.into()));
    }

    #[inline]
    pub fn scale(&mut self, x: f64, y: f64) {
        self.commands.push(Command::Scale(x, y));
    }

    #[inline]
    pub fn rotate(&mut self, angle: f64) {
        self.commands.push(Command::Rotate(angle));
    }

    #[inline]
    pub fn translate(&mut self, x: f64, y: f64) {
        self.commands.push(Command::Translate(x, y));
    }

    #[inline]
    pub fn stroke(&mut self) {
        self.commands.push(Command::Stroke);
    }

    #[inline]
    pub fn fill(&mut self) {
        self.commands.push(Command::Fill);
    }

    #[inline]
    pub fn paint(&mut self) {
        self.commands.push(Command::Paint);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context() {
        let mut cx = Context::new();
        cx.text("hello world");
        cx.line_to((4, 5));
        cx.rect((3, 6), 30, 50);
        cx.stroke();
        cx.fill();
        cx.image("image.png", (0, 0));
        cx.image_data(&[0x00, 0xFF, 0x55, 0x00], ImageFormat::Rgba8, (0, 0), 20, 20);
        cx.paint();

        for command in cx.commands() {
            println!("{:?}", command);
        }
    }
}
