/// Guibas and Stolfi implementation of the delaunay triangulation
use crate::edge::*;
use crate::robust_float::{counter_clockwise, in_circle, sanitize_points_vec, Point2};

fn compute_delaunay(quad_arena: &mut QuadEdgeArena, points: &[Point2]) -> (EdgeRef, EdgeRef) {
    if points.len() < 2 {
        panic!("Not enough points in vec!")
    }
    if points.len() == 2 {
        // println!(
        //     "Points length = 2, origin: {:?}, dest: {:?}",
        //     points[0], points[1]
        // );
        let mut a = make_edge(quad_arena);
        a.set_org(quad_arena, points[0]);
        a.set_dest(quad_arena, points[1]);
        return (a, a.sym());
    } else if points.len() == 3 {
        let mut a = make_edge(quad_arena);
        let mut b = make_edge(quad_arena);

        splice(quad_arena, a.sym(), b);
        a.set_org(quad_arena, points[0]);
        b.set_org(quad_arena, points[1]);
        a.set_dest(quad_arena, points[1]);
        b.set_dest(quad_arena, points[2]);

        if counter_clockwise(&points[0], &points[1], &points[2]) {
            let _ = connect(quad_arena, a, b);
            return (a, b.sym());
        } else if counter_clockwise(&points[0], &points[2], &points[1]) {
            let c = connect(quad_arena, b, a);
            return (c.sym(), c);
        } else {
            return (a, b.sym());
        }
    } else {
        // points.len() >= 4
        // Find the base left oriented edge
        let (mut ldo, mut ldi) = compute_delaunay(quad_arena, &points[..points.len() / 2]);
        let (mut rdi, mut rdo) = compute_delaunay(quad_arena, &points[points.len() / 2..]);
        loop {
            if left_of(quad_arena, &rdi.org(&quad_arena), ldi) {
                ldi = ldi.lnext(quad_arena);
            } else if right_of(quad_arena, &ldi.org(&quad_arena), rdi) {
                rdi = rdi.rprev(quad_arena);
            } else {
                break;
            }
        }
        let mut basel = connect(quad_arena, rdi.sym(), ldi);
        if ldi.org(&quad_arena) == ldo.org(&quad_arena) {
            ldo = basel.sym()
        }
        if rdi.org(&quad_arena) == rdo.org(&quad_arena) {
            rdo = basel
        }

        // Merge loop start
        loop {
            let mut lcand = basel.sym().onext(&quad_arena);
            if valid(quad_arena, lcand, basel) {
                while in_circle(
                    &basel.dest(&quad_arena),
                    &basel.org(&quad_arena),
                    &lcand.dest(&quad_arena),
                    &lcand.onext(&quad_arena).dest(&quad_arena),
                ) {
                    let t = lcand.onext(&quad_arena);
                    delete_edge(quad_arena, lcand);
                    lcand = t;
                }
            }

            let mut rcand = basel.oprev(quad_arena);
            if valid(quad_arena, rcand, basel) {
                while in_circle(
                    &basel.dest(&quad_arena),
                    &basel.org(&quad_arena),
                    &rcand.dest(&quad_arena),
                    &rcand.oprev(&quad_arena).dest(&quad_arena),
                ) {
                    let t = rcand.oprev(&quad_arena);
                    delete_edge(quad_arena, lcand);
                    rcand = t;
                }
            }

            if !valid(&quad_arena, lcand, basel) && !valid(quad_arena, rcand, basel) {
                break;
            }

            if !valid(quad_arena, lcand, basel)
                || (valid(quad_arena, rcand, basel)
                    && in_circle(
                        &lcand.dest(quad_arena),
                        &lcand.org(&quad_arena),
                        &rcand.org(&quad_arena),
                        &rcand.dest(&quad_arena),
                    ))
            {
                basel = connect(quad_arena, rcand, basel.sym());
            } else {
                basel = connect(quad_arena, basel.sym(), lcand.sym());
            }
        }
        return (ldo, rdo);
    }
}

/// Triangulate function returns a list of all lines
pub fn triangulate(points: &mut Vec<Point2>) -> Vec<(Point2, Point2)> {
    // Steps:
    // 1- Sort points
    // 2- Delete near-equal points
    sanitize_points_vec(points);
    // 3- triangulate
    let mut quad_arena = QuadEdgeArena::with_capacity(points.len() * 4); // Random ass big value
    compute_delaunay(&mut quad_arena, points);
    // 4- parse return values into data
    let mut lines = vec![];
    for (_, quad_edge) in quad_arena.iter() {
        if let Some((org, dest)) = quad_edge.get_points() {
            lines.push((org, dest));
        }
    }
    lines
}
