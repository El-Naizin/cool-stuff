use std::fmt;
use std::mem::MaybeUninit;
use std::ptr::addr_of_mut;

use id_arena::{Arena, Id};

use crate::robust_float::Point2;

type QuadEdgeId = Id<QuadEdge>;
type QuadEdgeArena = Arena<QuadEdge>;

#[derive(Copy, Clone, Debug)]
pub struct EdgeRef {
    quad_edge: QuadEdgeId,
    idx: usize,
}

#[derive(Debug)]
pub struct Edge {
    origin: Point2,
    next: EdgeRef,
}

#[derive(Debug)]
pub struct QuadEdge {
    edges: [Edge; 4],
    deleted: bool,
}

///
/// Create a QuadEdge allocated in the QuadEdgeArena, initialise the Edges of the QuadEdge with default
/// 0.0, 0.0 origin positions, and good edge_ref default values
///
pub fn make_edge(quad_arena: &mut QuadEdgeArena) -> EdgeRef {
    let quad_id = quad_arena.alloc(unsafe { MaybeUninit::<QuadEdge>::uninit().assume_init() });
    let quad_ptr = quad_arena.get_mut(quad_id).unwrap();

    let ref0 = EdgeRef {
        quad_edge: quad_id,
        idx: 0,
    };
    let ref1 = EdgeRef {
        quad_edge: quad_id,
        idx: 1,
    };
    let ref2 = EdgeRef {
        quad_edge: quad_id,
        idx: 2,
    };
    let ref3 = EdgeRef {
        quad_edge: quad_id,
        idx: 3,
    };

    unsafe {
        addr_of_mut!((*quad_ptr).deleted).write(false);
    }
    unsafe {
        addr_of_mut!((*quad_ptr).edges).write([
            Edge {
                origin: Point2::default(),
                next: ref0.clone(),
            },
            Edge {
                origin: Point2::default(),
                next: ref3.clone(),
            },
            Edge {
                origin: Point2::default(),
                next: ref2.clone(),
            },
            Edge {
                origin: Point2::default(),
                next: ref1.clone(),
            },
        ])
    }
    ref0
}

impl EdgeRef {
    ///////////////////////////
    // Dereferencing methods //
    ///////////////////////////
    pub fn quad_edge<'a, 'b>(&'a self, quad_arena: &'b QuadEdgeArena) -> &'b QuadEdge {
        quad_arena.get(self.quad_edge).unwrap()
    }

    pub fn quad_edge_mut<'a, 'b>(&'a self, quad_arena: &'b mut QuadEdgeArena) -> &'b mut QuadEdge {
        quad_arena.get_mut(self.quad_edge).unwrap()
    }

    pub fn raw_edge<'a, 'b>(&'a self, quad_arena: &'b QuadEdgeArena) -> &'b Edge {
        let quad = self.quad_edge(quad_arena);
        &quad.edges[self.idx]
    }

    pub fn raw_edge_mut<'a, 'b>(&'a self, quad_arena: &'b mut QuadEdgeArena) -> &'b mut Edge {
        let quad = self.quad_edge_mut(quad_arena);
        &mut quad.edges[self.idx]
    }

    ///////////////////////////////
    // Edge manipulation methods //
    ///////////////////////////////
    pub fn onext(&self, quad_arena: &QuadEdgeArena) -> EdgeRef {
        let edge = self.raw_edge(quad_arena);
        edge.next
    }

    pub fn set_onext(&self, quad_arena: &mut QuadEdgeArena, next: EdgeRef) {
        let edge = self.raw_edge_mut(quad_arena);
        edge.next = next;
    }

    pub fn oprev(&self, quad_arena: &QuadEdgeArena) -> EdgeRef {
        // edge.rot.next.rot
        self.rot().onext(quad_arena).rot()
    }

    /// Creates an Edgeref of the edge rotated once
    pub fn rot(&self) -> EdgeRef {
        EdgeRef {
            quad_edge: self.quad_edge,
            idx: (self.idx + 1) % 4,
        }
    }

    /// Creates an Edgeref of the edge rotated thrice
    pub fn inv_rot(&self) -> EdgeRef {
        EdgeRef {
            quad_edge: self.quad_edge,
            idx: (self.idx + 3) % 4,
        }
    }

    /// Creates an Edgeref of the edge rotated twice
    pub fn sym(&self) -> EdgeRef {
        EdgeRef {
            quad_edge: self.quad_edge,
            idx: (self.idx + 2) % 4,
        }
    }

    pub fn lnext(&self, quad_arena: &QuadEdgeArena) -> EdgeRef {
        self.inv_rot().onext(quad_arena).rot()
    }

    pub fn rprev(&self, quad_arena: &QuadEdgeArena) -> EdgeRef {
        self.sym().onext(quad_arena)
    }

    ////////////////////////////////
    // Coord manipulation methods //
    ////////////////////////////////
    pub fn org_dest(&self, quad_arena: &QuadEdgeArena) -> (Point2, Point2) {
        (self.org(quad_arena), self.dest(quad_arena))
    }

    pub fn org(&self, quad_arena: &QuadEdgeArena) -> Point2 {
        let edge = self.raw_edge(quad_arena);
        edge.origin
    }

    pub fn set_org(&mut self, quad_arena: &mut QuadEdgeArena, vert: Point2) {
        let mut edge = self.raw_edge_mut(quad_arena);
        edge.origin = vert;
    }

    pub fn dest(&self, quad_arena: &QuadEdgeArena) -> Point2 {
        self.sym().org(quad_arena)
    }

    pub fn set_dest(&mut self, quad_arena: &mut QuadEdgeArena, vert: Point2) {
        self.sym().set_org(quad_arena, vert);
    }
}

impl fmt::Display for EdgeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Id: {}, Index: {})", self.quad_edge.index(), self.idx)
    }
}

pub fn splice(quad_arena: &mut QuadEdgeArena, a: EdgeRef, b: EdgeRef) {
    let alpha = a.onext(&quad_arena).rot();
    let beta = b.onext(&quad_arena).rot();

    let a_next = a.onext(&quad_arena);
    let b_next = b.onext(&quad_arena);
    let alpha_next = alpha.onext(&quad_arena);
    let beta_next = beta.onext(&quad_arena);

    a.set_onext(quad_arena, b_next);
    b.set_onext(quad_arena, a_next);
    alpha.set_onext(quad_arena, beta_next);
    beta.set_onext(quad_arena, alpha_next);
}

pub fn swap(quad_arena: &mut QuadEdgeArena, edge: &mut EdgeRef) {
    let a = edge.oprev(&quad_arena);
    let b = edge.sym().oprev(quad_arena);
    splice(quad_arena, edge.to_owned(), a);
    splice(quad_arena, edge.sym(), b);
    splice(quad_arena, edge.to_owned(), a.lnext(quad_arena));
    splice(quad_arena, edge.sym(), b.lnext(quad_arena));
    edge.set_org(quad_arena, a.dest(quad_arena));
    edge.set_dest(quad_arena, b.dest(quad_arena));
}

pub fn connect(quad_arena: &mut QuadEdgeArena, a: EdgeRef, b: EdgeRef) -> EdgeRef {
    let mut edge = make_edge(quad_arena);
    edge.set_org(quad_arena, a.dest(quad_arena));
    edge.set_dest(quad_arena, b.org(quad_arena));
    splice(quad_arena, edge, a.lnext(quad_arena));
    splice(quad_arena, edge.sym(), b.to_owned());
    edge
}

pub fn delete_edge(quad_arena: &mut QuadEdgeArena, edge: EdgeRef) {
    splice(quad_arena, edge, edge.oprev(quad_arena));
    splice(quad_arena, edge.sym(), edge.sym().oprev(quad_arena));
    let quad_edge = quad_arena.get_mut(edge.quad_edge).unwrap();
    quad_edge.deleted = true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_edge() {
        let mut quad_arena = QuadEdgeArena::new();
        let edge_ref = make_edge(&mut quad_arena);
        println!("Edge ref: {}", edge_ref);
        assert_eq!(1, 1);
    }
}
