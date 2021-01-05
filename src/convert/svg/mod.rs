//! Convert from and into an SVG image

mod elements;
mod object;

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_test()
    {
        let svg = r#"
            <svg>
                <line x1="4" y1="4" x2="4" y2="4" />
                <rect width="100" height="100" />
                <line x1="0" y1="0" x2="3" y2="3" />
                <text>hello world</text>
            </svg>
        "#;

        println!("{:?}", into::string(svg).unwrap());
    }
}
