use core::f64;
use robust;
use std::fmt::Display;

/// Points have top-right x,y coordinates:
/// 0,0 ------- 1,0
///  |           |
///  |           |
///  |           |
/// 0,1 ------- 1,1
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Point2 {
    x: f64,
    y: f64,
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
pub fn in_circle(a: &Point2, b: &Point2, c: &Point2, d: &Point2) -> bool {
    robust::incircle(a.into(), b.into(), c.into(), d.into()) < 0.
}

#[inline(always)]
pub fn counter_clockwise(a: &Point2, b: &Point2, c: &Point2) -> bool {
    // println!(
    //     "{} {} {}: {}",
    //     a,
    //     b,
    //     c,
    //     robust::orient2d(a.into(), b.into(), c.into())
    // );
    robust::orient2d(a.into(), b.into(), c.into()) < 0.
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
}
