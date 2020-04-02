//! # Surface
//! A surface contains a set of object
//! that are intented to be displayed.

#[allow(unused_imports)]

use std::collections::HashMap;
use crate::object::*;
use crate::object::text::Text;

/// A position on the surface
pub type Position = (usize, usize);

/// The type of position
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum PositionType {
    /// The absolute position
    Abs,
    /// The relative position
    Rel
}

pub type Map<K, V> = HashMap<K, V>;

/// Meta data about objects on the surface
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Meta {
    /// The position within the surface
    pub pos: Position,
    pub postype: PositionType
}

impl Meta {
    pub fn new() -> Self
    {
        Self {
            pos: (0, 0),
            postype: PositionType::Abs
        }
    }
}

/// Group multiple objects together
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Group {
    pub meta: Meta,
    objects: Vec<Object>
}

impl Group {
    pub fn new(objects: Vec<Object>) -> Self
    {
        Self {
            meta: Meta::new(),
            objects
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Primitive {
    Point(Point),
    Line(Line),
    Rect(Rect),
    Text(Text)
}

impl Translate for Primitive {
    fn point(&self) -> &Point
    {
        use Primitive::*;
        match self {
            Point(p) => p.point(),
            Line(l) => l.point(),
            Rect(r) => r.point(),
            Text(t) => t.point()
        }
    }

    fn points(&self) -> Option<&[Point]>
    {
        use Primitive::*;
        match self {
            Point(p) => p.points(),
            Line(l) => l.points(),
            Rect(r) => r.points(),
            Text(t) => t.points()
        }
    }

    fn point_mut(&mut self) -> &mut Point
    {
        use Primitive::*;
        match self {
            Point(p) => p.point_mut(),
            Line(l) => l.point_mut(),
            Rect(r) => r.point_mut(),
            Text(t) => t.point_mut()
        }
    }

    fn points_mut(&mut self) -> Option<&mut [Point]>
    {
        use Primitive::*;
        match self {
            Point(p) => p.points_mut(),
            Line(l) => l.points_mut(),
            Rect(r) => r.points_mut(),
            Text(t) => t.points_mut()
        }
    }
}

impl Scale for Primitive {}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Object {
    Primitive(Primitive),
    Group(Group)
}

/// A surface contains a set of object
/// that are intented to be displayed.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Surface {
    meta: Meta,
    objects: Vec<Object>
}

impl Surface {
    pub fn new(objects: Vec<Object>) -> Self
    {
        Self {
            meta: Meta::new(),
            objects
        }
    }

    pub fn for_each<F>(&self, f: F)
        where F: Fn(&Object)
    {
        self.objects.iter().for_each(f);
    }

    pub fn for_each_mut<F>(&self, mut f: F)
        where F: FnMut(&Object)
    {
        self.objects.iter().for_each(f);
    }
}
