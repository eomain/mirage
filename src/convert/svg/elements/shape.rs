
use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "line")]
pub struct Line {
    pub x1: isize,
    pub y1: isize,
    pub x2: isize,
    pub y2: isize,
}

impl Line {
    pub fn new(x1: isize, y1: isize, x2: isize, y2: isize) -> Self
    {
        Self {
            x1, y1, x2, y2
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "line")]
pub struct PolyLine {
    #[serde(default)]
    points: String
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "rect")]
pub struct Rect {
    pub x: Option<isize>,
    pub y: Option<isize>,
    pub rx: Option<isize>,
    pub ry: Option<isize>,
    pub width: usize,
    pub height: usize
}

impl Rect {
    pub fn new(x: Option<isize>, y: Option<isize>, rx: Option<isize>,
               ry: Option<isize>, width: usize, height: usize) -> Self
    {
        Self {
            x, y,
            rx, ry,
            width,
            height
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Shape {
    #[serde(rename = "line")]
    Line(Line),
    #[serde(rename = "rect")]
    Rect(Rect)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line()
    {
        let svg = r#"
            <line x1="0" y1="0" x2="3" y2="3" />
        "#;
        let l: Line = from_str(&svg).unwrap();
        assert_eq!(l, Line::new(0, 0, 3, 3));
    }

    #[test]
    fn point()
    {
        let svg = r#"
            <polyline points="20,20 40,25 60,40 80,120 120,140 200,180" />
        "#;
        let p: PolyLine = from_str(&svg).unwrap();
        println!("{:?}", p);
    }

    #[test]
    fn rect()
    {
        let svg = r#"
            <rect width="100" height="100" />
        "#;
        let r: Rect = from_str(&svg).unwrap();
        assert_eq!(r, Rect::new(None, None, None, None, 100, 100));
    }

    #[test]
    fn shape()
    {
        let svg = r#"
            <line x1="5" y1="2" x2="12" y2="20" />
            <rect width="300" height="200" />
        "#;
        let s: Shape = from_str(&svg).unwrap();
        assert_eq!(s, Shape::Line(Line::new(5, 2, 12, 20)));
    }
}
