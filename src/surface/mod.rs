//! # Surface
//! A surface contains a set of object
//! that are intented to be displayed.

#[allow(unused_imports)]

use std::collections::HashMap;

/// A position on the surface
pub type Position = (usize, usize);

pub type Map<K, V> = HashMap<K, V>;

/// A surface contains a set of object
/// that are intented to be displayed.
#[derive(Clone, Hash)]
pub struct Surface {

}
