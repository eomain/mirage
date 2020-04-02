
use crate::object::*;
use crate::object::text::Text;
use crate::surface;
use surface::{
    Object,
    Surface
};
use super::*;
use elements::Svg;
use elements::shape;

fn point(l: &shape::Line) -> Point
{
    Point::new(l.x1, l.y1)
}

fn line(l: &shape::Line) -> Line
{
    line![
        (l.x1, l.y1),
        (l.x2, l.y2)
    ]
}

fn rect(r: &shape::Rect) -> Rect
{
    Rect::new(match (r.x, r.y) {
        (None, None) => (0, 0),
        (Some(x), None) => (x, 0),
        (None, Some(y)) => (0, y),
        (Some(x), Some(y)) => (x, y)
    }, r.width, r.height)
}

fn text(t: &shape::Text) -> Text
{
    Text::new((t.x, t.y), &t.text)
}

pub fn svg(s: &Svg) -> Surface
{
    let mut v = Vec::new();

    use shape::Shape;
    use surface::Primitive::*;
    use Object::*;

    for s in &s.shapes {
        v.push(
            match s {
                Shape::Line(l) => {
                    if l.x1 == l.x2 && l.y1 == l.y2 {
                        continue;
                    } else if (l.x1 - l.x2).abs() == 1 && (l.y1 - l.y2).abs() == 1 {
                        Primitive(Point(point(l)))
                    } else {
                        Primitive(Line(line(l)))
                    }
                },
                Shape::Rect(r) => {
                    Primitive(Rect(rect(r)))
                },
                Shape::Text(t) => {
                    Primitive(Text(text(t)))
                }
            }
        );
    }

    Surface::new(v)
}

#[cfg(test)]
mod tests {
    extern crate serde;
    extern crate serde_xml_rs as serde_xml;

    use serde_xml_rs::from_str as from;

    use super::*;

    #[test]
    fn point_test()
    {
        let svg = r#"
            <line x1="4" y1="8" x2="4" y2="8" />
        "#;
        let p = point(&from(svg).unwrap());
        assert_eq!(p, Point::new(4, 8));
    }

    #[test]
    fn line_test()
    {
        let svg = r#"
            <line x1="0" y1="0" x2="3" y2="3" />
        "#;
        let l = line(&from(svg).unwrap());
        assert_eq!(l, line![(0, 0), (3, 3)]);
    }

    #[test]
    fn rect_test()
    {
        let svg = r#"
            <rect x="3" y="6" width="300" height="200" />
        "#;
        let r = rect(&from(svg).unwrap());
        assert_eq!(r, Rect::new((3, 6), 300, 200));
    }
}
