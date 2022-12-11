use core::f64;
use robust;
use std::fmt::Display;

const EPSILON: f64 = f64::EPSILON * 2.;

/// Points have top-right x,y coordinates:
/// 0,0 ------- 1,0
///  |           |
///  |           |
///  |           |
/// 0,1 ------- 1,1
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

impl Display for Point2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {},y: {})", self.x, self.y)
    }
}

impl From<&Point2> for robust::Coord<f64> {
    fn from(p: &Point2) -> robust::Coord<f64> {
        robust::Coord::<f64> { x: p.x, y: p.y }
    }
}

#[inline(always)]
pub fn nearly_equals(a: &Point2, b: &Point2) -> bool {
    (a.x - b.x).abs() <= EPSILON && (a.y - b.y).abs() <= EPSILON
}

#[inline(always)]
pub fn in_circle(a: &Point2, b: &Point2, c: &Point2, d: &Point2) -> bool {
    robust::incircle(a.into(), b.into(), c.into(), d.into()) < 0.
}

#[inline(always)]
pub fn counter_clockwise(a: &Point2, b: &Point2, c: &Point2) -> bool {
    robust::orient2d(a.into(), b.into(), c.into()) < 0.
}

pub fn sort_points(points: &mut Vec<Point2>) {
    points.sort_by(|a, b| match a.x.partial_cmp(&b.x) {
        Some(ord) => match ord {
            std::cmp::Ordering::Equal => a.y.partial_cmp(&b.y).unwrap(),
            x => x,
        },
        None => a.y.partial_cmp(&b.y).unwrap(),
    });
}

fn remove_near_equal_points(points: &mut Vec<Point2>) {
    let mut idx = 0;
    while idx < points.len() - 1 {
        if nearly_equals(&points[idx], &points[idx + 1]) {
            points.remove(idx);
        } else {
            idx += 1;
        }
    }
}

pub fn sanitize_points_vec(points: &mut Vec<Point2>) {
    sort_points(points);
    remove_near_equal_points(points);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_circle() {
        let a = Point2 { x: 0., y: 0. };
        let b = Point2 { x: 0., y: 1. };
        let c = Point2 { x: 1., y: 0. };
        let d = Point2 { x: 1., y: 1. };
        let e = Point2 { x: 0.5, y: 0.5 };
        assert_eq!(in_circle(&a, &b, &c, &d), false);
        assert_eq!(in_circle(&a, &b, &c, &e), true);
    }

    #[test]
    fn test_counter_clockwise() {
        let a = Point2 { x: 0., y: 0. };
        let b = Point2 { x: 0., y: 1. };
        let c = Point2 { x: 1., y: 0. };
        let d = Point2 { x: 1., y: 1. };
        let e = Point2 { x: 0.5, y: 0.5 };
        assert_eq!(counter_clockwise(&a, &b, &c), true);
        assert_eq!(counter_clockwise(&a, &c, &b), false);
        assert_eq!(counter_clockwise(&c, &b, &a), false);
        assert_eq!(counter_clockwise(&b, &c, &a), true);
        assert_eq!(counter_clockwise(&e, &c, &a), true);
        assert_eq!(counter_clockwise(&e, &d, &a), false);
    }

    #[test]
    fn test_remove_near_equal_points() {
        let mut points = vec![
            Point2 { x: 0., y: 1. },
            Point2 { x: 0., y: 1. },
            Point2 { x: 1., y: 1. },
            Point2 { x: 3., y: 1. },
            Point2 { x: 3., y: 1. },
            Point2 { x: 3., y: 2. },
        ];

        sanitize_points_vec(&mut points);
        assert_eq!(
            points,
            vec![
                Point2 { x: 0., y: 1. },
                Point2 { x: 1., y: 1. },
                Point2 { x: 3., y: 1. },
                Point2 { x: 3., y: 2. },
            ]
        );
    }
}
