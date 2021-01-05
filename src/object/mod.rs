//! Vector graphics type primitives

pub mod bitmap;
/// Contains objects for the creation
/// of graphical text
pub mod text;

use std::ops::Add;

/// Perform a translation on an Object
/// by operating on their points
pub trait Translate {

    fn point(&self) -> &Point;

    fn points(&self) -> Option<&[Point]>
    {
        None
    }

    fn point_mut(&mut self) -> &mut Point;

    fn points_mut(&mut self) -> Option<&mut [Point]>
    {
        None
    }

    fn for_each<F>(&self, func: F)
        where F: Fn(& Point)
    {
        match self.points() {
            None => {
                func(self.point());
            },
            Some(points) => {
                points.iter().for_each(|p| func(p));
            }
        }
    }

    fn for_each_mut<F>(&mut self, mut func: F)
        where F: FnMut(&mut Point)
    {
        match self.points_mut() {
            None => {
                func(self.point_mut());
            },
            Some(points) => {
                points.iter_mut().for_each(|p| func(p));
            }
        }
    }

    fn position(&mut self, pos: (isize, isize))
    {
        let (x, y) = self.point().into();
        self.translate((pos.0 - x, pos.1 - y));
    }

    fn translate_x(&mut self, pos: isize)
    {
        self.for_each_mut(|point| {
            point.x += pos;
        });
    }

    fn translate_y(&mut self, pos: isize)
    {
        self.for_each_mut(|point| {
            point.y += pos;
        });
    }

    fn translate(&mut self, pos: (isize, isize))
    {
        self.for_each_mut(|point| {
            point.x += pos.0;
            point.y += pos.1;
        });
    }

}

pub trait Scale: Translate {
    fn scale_x(&mut self, factor: f64)
    {
        self.for_each_mut(|point| {
            let x = -(0.0 - point.x as f64);
            point.x = (factor * x) as isize;
        });
    }

    fn scale_y(&mut self, factor: f64)
    {
        self.for_each_mut(|point| {
            let y = -(0.0 - point.y as f64);
            point.y = (factor * y) as isize;
        });
    }

    fn scale(&mut self, factor: f64)
    {
        self.for_each_mut(|point| {
            let x = -(0.0 - point.x as f64);
            let y = -(0.0 - point.y as f64);
            point.x = (factor * x) as isize;
            point.y = (factor * y) as isize;
        });
    }
}

/// A Point is a simple object that
/// represents a single location
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
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

    pub(crate) fn set_max(&mut self, other: &Self)
    {
        if other.x > self.x {
            self.x = other.x;
        }

        if other.y > self.y {
            self.y = other.y;
        };
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output
    {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Translate for Point {
    fn point(&self) -> &Point
    {
        self
    }

    fn point_mut(&mut self) -> &mut Point
    {
        self
    }
}

impl Scale for Point {}

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
    /// The begin point of the line(s)
    pub begin: Point,
    /// All of the relative points of the lines path
    pub path: Vec<Point>
}

impl Line {
    pub fn new<P>(begin: P, path: Vec<Point>) -> Self
        where P: Into<Point>
    {
        Self {
            begin: begin.into(),
            path,
        }
    }

    pub fn path(&self) -> Vec<Point>
    {
        let mut point = self.begin;
        let mut v = vec![point];
        for p in &self.path {
            point = point + *p;
            v.push(point);
        }
        v
    }
}

impl Translate for Line {
    fn point(&self) -> &Point
    {
        &self.begin
    }

    fn point_mut(&mut self) -> &mut Point
    {
        &mut self.begin
    }
}

impl Scale for Line {}

impl From<&[Point]> for Line {
    fn from(p: &[Point]) -> Self
    {
        assert!(p.len() > 0);
        let mut points = p.to_vec();
        Self::new(points.remove(0), points)
    }
}

impl IntoIterator for Line {
    type Item = Point;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.path().into_iter()
    }
}

/// Create a line from a sequence of points
#[macro_use]
#[macro_export]
macro_rules! line {
    () => {
        Line::new((0, 0), Vec::new())
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

impl Translate for Rect {
    fn point(&self) -> &Point
    {
        &self.point
    }

    fn point_mut(&mut self) -> &mut Point
    {
        &mut self.point
    }
}

impl Scale for Rect {}

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
        let mut p: Point = (2, 4).into();
        assert_eq!(p.x, 2);
        assert_eq!(p.y, 4);
        p.scale(3.5);
        assert_eq!(p.x, 7);
        assert_eq!(p.y, 14);
    }

    #[test]
    fn line()
    {
        let mut l = line![(2, 1), (1, 1), (5, 5)];
        l.position((3, 4));
    }

    #[test]
    fn rect()
    {
        let r = Rect::new((4, 5), 3, 3);
    }

}
