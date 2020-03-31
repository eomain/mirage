//! # Objects
//! In mirage, all graphics are represented as
//! a collection of Objects.

/// Contains objects for the creation
/// of graphical text
pub mod text;

/// A Point is a simple object that
/// represents a single location
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct Point {
    /// The x coordinate
    pub x: isize,
    /// The y coordinate
    pub y: isize
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self
    {
        Self {
            x, y
        }
    }
}

impl From<(isize, isize)> for Point {

    fn from(p: (isize, isize)) -> Self
    {
        Self::new(p.0, p.1)
    }

}

impl From<&Point> for (isize, isize) {

    fn from(p: &Point) -> Self
    {
        (p.x, p.y)
    }
}

/// A sequence of Points that form a line(s)
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Line {
    /// All of the points of the lines path
    pub points: Vec<Point>
}

impl Line {
    pub fn new() -> Self
    {
        Self {
            points: Vec::new()
        }
    }
}

impl From<&[Point]> for Line {
    fn from(p: &[Point]) -> Self
    {
        Self {
            points: p.to_vec()
        }
    }
}

impl<'a> IntoIterator for &'a Line {
    type Item = &'a Point;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.points.iter()
                   .collect::<Vec<_>>()
                   .into_iter()
    }
}

/// Create a line from a sequence of points
#[macro_use]
#[macro_export]
macro_rules! line {
    () => {
        Line::new()
    };

    ($x: expr) => {
        {
            let point: &[Point] = &[
                $x.into()
            ];

            Line::from(point)
        }
    };

    ($x: expr, $($y: expr), *) => {
        {
            let points: &[Point] = &[
                $x.into(),
                $(
                    $y.into(),
                )*
            ];

            Line::from(points)
        }
    };
}

/// A rectangular area with an origin (Point),
/// as well as a width and height
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Rect {
    /// The origin of the rectangle
    pub point: Point,
    /// The width of the rectangle
    pub width: usize,
    /// The height of the rectangle
    pub height: usize
}

impl Rect {
    pub fn new<P>(p: P, width: usize, height: usize) -> Self
        where P: Into<Point>
    {
        Self {
            point: p.into(),
            width,
            height
        }
    }
}

impl<'a> From<&'a Rect> for (&'a Point, usize, usize)
{
    fn from(rect: &'a Rect) -> Self
    {
        (&rect.point, rect.width, rect.height)
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_variables)]
    use super::*;

    #[test]
    fn point()
    {
        let p: Point = (2, 4).into();
        assert_eq!(p.x, 2);
        assert_eq!(p.y, 4);
    }

    #[test]
    fn line()
    {
        let l = line![(2, 1), (1, 1)];
    }

    #[test]
    fn rect()
    {
        let r = Rect::new((4, 5), 3, 3);
    }

}
