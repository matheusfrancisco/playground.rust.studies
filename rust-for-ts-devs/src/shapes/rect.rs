use std::fmt::Display;

use super::area::Area;

pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Area for Rect {
    fn area(&self) -> f64 {
        self.height * self.width
    }
}

impl Default for Rect {
    fn default() -> Self {
        Rect {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        }
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rectangle({}, {}): {}x{}",
            self.x, self.y, self.width, self.height
        )
    }
}

pub struct RectIter {
    points: Vec<(f64, f64)>,
    idx: usize,
}

impl Iterator for RectIter {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;

        self.points.get(idx).map(|x| *x)
    }
}

//impl RectIter {
//    fn new(rect: &Rect) -> Self {
//        RectIter {
//            points: vec![
//                (rect.x, rect.y),
//                (rect.x + rect.width, rect.y),
//                (rect.x, rect.y + rect.height),
//                (rect.x + rect.width, rect.y + rect.height),
//            ],
//            idx: 0,
//        }
//    }
//}

impl From<&Rect> for RectIter {
    fn from(rect: &Rect) -> Self {
        RectIter {
            points: vec![
                (rect.x, rect.y),
                (rect.x + rect.width, rect.y),
                (rect.x, rect.y + rect.height),
                (rect.x + rect.width, rect.y + rect.height),
            ],
            idx: 0,
        }
    }
}

impl IntoIterator for Rect {
    type Item = (f64, f64);
    type IntoIter = RectIter;

    fn into_iter(self) -> Self::IntoIter {
        return (&self).into();
    }
}

impl IntoIterator for &Rect {
    type Item = (f64, f64);
    type IntoIter = RectIter;

    fn into_iter(self) -> Self::IntoIter {
        return self.into();
    }
}
