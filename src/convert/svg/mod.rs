
mod elements;
mod object;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Parse
}

pub mod into {

    extern crate serde;
    extern crate serde_xml_rs as serde_xml;

    use crate::surface::Surface;
    use super::*;
    use object::svg;

    use serde_xml_rs::from_str as from;

    pub fn string(s: &str) -> Result<Surface, Error>
    {
        match from(s) {
            Err(_) => Err(Error::Parse),
            Ok(s) => Ok(svg(&s))
        }
    }
}
