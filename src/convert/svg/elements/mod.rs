
pub mod shape;

use shape::Shape;
use serde::{Serialize, Deserialize};
use serde_xml_rs::{from_str, from_reader, to_string};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "svg")]
pub struct Svg {
    #[serde(default)]
    pub x: isize,
    #[serde(default)]
    pub y: isize,
    #[serde(rename = "$value", default)]
    pub shapes: Vec<Shape>
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::shape::*;

    #[test]
    fn svg()
    {
        let svg = r#"
            <svg>
                <line x1="0" y1="0" x2="3" y2="3" />
                <rect width="100" height="100" />
                <line x1="0" y1="0" x2="3" y2="3" />
            </svg>
        "#;
        let s: Svg = from_str(&svg).unwrap();
        println!("{:?}", s);
    }
}
