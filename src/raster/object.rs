
use super::*;
use crate::object::*;
use crate::surface::Object;

struct ScaleFactor {
    factor: f64
}

impl ScaleFactor {
    fn new(factor: f64) -> Self
    {
        Self {
            factor
        }
    }
}

fn point(p: &Point) -> (usize, usize)
{
    let (x, y) = p.into();
    (x as usize, y as usize)
}

fn point_to_point(p1: (usize, usize), p2: (usize, usize)) -> Vec<(usize, usize)>
{
    let mut v = Vec::new();
    if p1.0 == p2.0 {
        let range = if p1.1 < p2.1 {
            (p1.1..=p2.1)
        } else {
            (p2.1..=p1.1)
        };

        let mut points = range.map(|y| (p1.0, y)).collect();
        v.append(&mut points);
    } else if p1.1 == p2.1 {
        let range = if p1.0 < p2.0 {
            (p1.0..=p2.0)
        } else {
            (p2.0..=p1.0)
        };

        let mut points = range.map(|x| (x, p1.1)).collect();
        v.append(&mut points);
    } else {
        let range = (p1.0..=p2.0);
        let dy = if p1.1 > p2.1 { p1.1 - p2.1 } else { p2.1 - p1.1 };
        let dx = if p1.0 > p2.0 { p1.0 - p2.0 } else { p2.0 - p1.0 };
        let m = (dy as f64 / dx as f64);
        let c = p1.1 - (m * p1.0 as f64) as usize;
        let mut points = range.map(|x| (x, (m * x as f64) as usize + c)).collect();
        v.append(&mut points);
    }
    v
}

fn line(l: &Line) -> Vec<(usize, usize)>
{
    let mut v = Vec::new();
    let points = l.points().unwrap();
    assert!(points.len() > 1);
    let mut p1 = point(&points[0]);
    for i in 1..points.len() {
        let p2 = point(&points[i]);
        v.append(&mut point_to_point(p1, p2));
        p1 = p2;
    }
    v
}

fn rect(r: &Rect) -> Vec<(usize, usize)>
{
    let mut v = Vec::new();
    let p = point(&r.point);
    let (w, h) = (r.width, r.height);

    let (tl, tr) = (p, (p.0 + w, p.1));
    let (bl, br) = ((p.0, p.1 + h), (p.0 + w, p.1 + h));

    let lines = [
        (tl, tr),
        (bl, br),
        (tl, bl),
        (tr, br)
    ];

    for l in &lines {
        v.append(&mut point_to_point(l.0, l.1));
    }
    v
}

pub fn object(o: &Object) -> Vec<(usize, usize)>
{
    use Object::*;
    match o {
        Point(p) => vec![point(p)],
        Line(l) => line(l),
        Rect(r) => rect(r),
        _ => unimplemented!()
    }
}
