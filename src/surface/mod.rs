//! # Surface
//! A surface contains a set of object
//! that are intented to be displayed.

#[allow(unused_imports)]

use std::collections::HashMap;
use crate::object::*;
use crate::object::text::Text;

/// A position on the surface
pub type Position = (usize, usize);

pub type Map<K, V> = HashMap<K, V>;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Object {
    Point(Point),
    Line(Line),
    Rect(Rect),
    Text(Text)
}

/// A surface contains a set of object
/// that are intented to be displayed.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Surface {
    objects: Vec<Object>
}

impl Surface {
    pub fn new(objects: Vec<Object>) -> Self
    {
        Self {
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
