//! Free categories over directed graphs.
//!
//! The free category `F(G)` on a graph `G` has:
//! - Objects = vertices of `G`
//! - Morphisms = directed paths in `G` (including the empty path)
//! - Composition = path concatenation
//! - Identity = empty path
//!
//! The universal property: any graph morphism from `G` into the
//! underlying graph of a category `C` extends uniquely to a functor
//! `F(G) -> C`.  This makes `F` left adjoint to the forgetful functor
//! `U: Cat -> Graph`, hence `F = Lan_η(Id)`, a left Kan extension.
//!
//! In halo2:
//! - Vertices = wire types (field elements, booleans, etc.)
//! - Edges = primitive gates (addition, multiplication, lookup, etc.)
//! - Paths = composed circuits
//! - The universal property = circuit interpretation / evaluation

/// A vertex identifier in a graph.
///
/// Newtype over `usize` to prevent confusion with edge indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vertex(usize);

impl Vertex {
    /// Create a new vertex identifier.
    #[must_use]
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    /// The underlying index.
    #[must_use]
    pub fn index(self) -> usize {
        self.0
    }
}

/// An edge identifier in a graph.
///
/// Newtype over `usize` to prevent confusion with vertex indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge(usize);

impl Edge {
    /// Create a new edge identifier.
    #[must_use]
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    /// The underlying index.
    #[must_use]
    pub fn index(self) -> usize {
        self.0
    }
}

/// Errors arising from free category operations.
#[derive(Debug)]
pub enum FreeCategoryError {
    /// An edge index was out of bounds.
    EdgeOutOfBounds {
        /// The invalid edge.
        edge: Edge,
        /// The number of edges in the graph.
        count: usize,
    },
    /// A vertex index was out of bounds.
    VertexOutOfBounds {
        /// The invalid vertex.
        vertex: Vertex,
        /// The number of vertices in the graph.
        count: usize,
    },
    /// Path composition failed: target of first path does not
    /// match source of second path.
    CompositionMismatch {
        /// The target of the first path.
        target: Vertex,
        /// The source of the second path.
        source: Vertex,
    },
}

impl core::fmt::Display for FreeCategoryError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EdgeOutOfBounds { edge, count } => {
                write!(f, "edge index {} out of bounds (count: {count})", edge.0)
            }
            Self::VertexOutOfBounds { vertex, count } => {
                write!(
                    f,
                    "vertex index {} out of bounds (count: {count})",
                    vertex.0
                )
            }
            Self::CompositionMismatch { target, source } => {
                write!(
                    f,
                    "composition mismatch: target vertex {} != source vertex {}",
                    target.0, source.0
                )
            }
        }
    }
}

impl std::error::Error for FreeCategoryError {}

/// A directed graph: the generating data for a free category.
///
/// A graph consists of a set of vertices, a set of edges, and
/// source/target functions mapping each edge to its endpoints.
pub trait Graph {
    /// The number of vertices.
    fn vertex_count(&self) -> usize;

    /// The number of edges.
    fn edge_count(&self) -> usize;

    /// The source vertex of an edge.
    ///
    /// # Errors
    ///
    /// Returns [`FreeCategoryError::EdgeOutOfBounds`] if the edge
    /// index is out of range.
    fn source(&self, edge: Edge) -> Result<Vertex, FreeCategoryError>;

    /// The target vertex of an edge.
    ///
    /// # Errors
    ///
    /// Returns [`FreeCategoryError::EdgeOutOfBounds`] if the edge
    /// index is out of range.
    fn target(&self, edge: Edge) -> Result<Vertex, FreeCategoryError>;
}

/// A path in the free category: a morphism from `source` to `target`.
///
/// The empty path (no edges) is the identity morphism.
/// A non-empty path is a sequence of composable edges.
///
/// Invariant: for a non-empty path with edges `[e0, e1, ..., en]`:
/// - `graph.source(e0) == self.source()`
/// - `graph.target(ei) == graph.source(e(i+1))` for consecutive pairs
/// - `graph.target(en) == self.target()`
#[must_use]
pub struct Path {
    source: Vertex,
    edges: Vec<Edge>,
    target: Vertex,
}

impl Path {
    /// The identity path on a vertex (empty path).
    pub fn identity(v: Vertex) -> Self {
        Self {
            source: v,
            edges: Vec::new(),
            target: v,
        }
    }

    /// A path consisting of a single edge.
    ///
    /// # Errors
    ///
    /// Returns [`FreeCategoryError::EdgeOutOfBounds`] if the edge is
    /// not in the graph.
    pub fn singleton(graph: &dyn Graph, edge: Edge) -> Result<Self, FreeCategoryError> {
        let src = graph.source(edge)?;
        let tgt = graph.target(edge)?;
        Ok(Self {
            source: src,
            edges: vec![edge],
            target: tgt,
        })
    }

    /// The source vertex of this path.
    #[must_use]
    pub fn source(&self) -> Vertex {
        self.source
    }

    /// The target vertex of this path.
    #[must_use]
    pub fn target(&self) -> Vertex {
        self.target
    }

    /// The edges in this path.
    #[must_use]
    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    /// The length of this path (number of edges).
    #[must_use]
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    /// Whether this path is empty (the identity morphism).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    /// Whether this is the identity (empty) path.
    ///
    /// Alias for [`is_empty`](Self::is_empty) using categorical terminology.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.is_empty()
    }

    /// Compose two paths: `self` followed by `other`.
    ///
    /// Requires: `self.target() == other.source()`
    ///
    /// # Errors
    ///
    /// Returns [`FreeCategoryError::CompositionMismatch`] if the
    /// target of `self` does not equal the source of `other`.
    pub fn compose(self, other: Self) -> Result<Self, FreeCategoryError> {
        if self.target == other.source {
            Ok(Self {
                source: self.source,
                edges: self
                    .edges
                    .into_iter()
                    .chain(other.edges)
                    .collect(),
                target: other.target,
            })
        } else {
            Err(FreeCategoryError::CompositionMismatch {
                target: self.target,
                source: other.source,
            })
        }
    }
}

/// A graph morphism: maps vertices to objects and edges to morphisms.
///
/// This is the "interpretation" of graph generators into a target
/// structure.  The universal property of the free category says
/// that any graph morphism extends uniquely to a functor.
pub trait GraphMorphism<G: Graph> {
    /// The target type for vertices (objects in the target category).
    type Object;

    /// The target type for edges (morphisms in the target category).
    type Morphism;

    /// Map a vertex to an object.
    fn map_vertex(&self, v: Vertex) -> Self::Object;

    /// Map an edge to a morphism.
    fn map_edge(&self, e: Edge) -> Self::Morphism;
}

/// Interpret a path in the free category via a graph morphism.
///
/// This is the universal property: the unique functor extending
/// the graph morphism to the free category.
///
/// For the identity path, returns `id_fn(source_object)`.
/// For a non-empty path, folds the interpreted edges via `comp_fn`.
#[must_use]
pub fn interpret<G, M>(
    morphism: &M,
    path: &Path,
    id_fn: impl Fn(&M::Object) -> M::Morphism,
    comp_fn: impl Fn(M::Morphism, M::Morphism) -> M::Morphism,
) -> M::Morphism
where
    G: Graph,
    M: GraphMorphism<G>,
{
    path.edges()
        .split_first()
        .map_or_else(
            || {
                let obj = morphism.map_vertex(path.source());
                id_fn(&obj)
            },
            |(first, rest)| {
                let init = morphism.map_edge(*first);
                rest.iter().fold(init, |acc, e| comp_fn(acc, morphism.map_edge(*e)))
            },
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A simple test graph: two vertices, one edge.
    ///
    /// ```text
    ///   0 --e0--> 1
    /// ```
    struct TwoVertexGraph;

    impl Graph for TwoVertexGraph {
        fn vertex_count(&self) -> usize {
            2
        }

        fn edge_count(&self) -> usize {
            1
        }

        fn source(&self, edge: Edge) -> Result<Vertex, FreeCategoryError> {
            match edge.index() {
                0 => Ok(Vertex::new(0)),
                _ => Err(FreeCategoryError::EdgeOutOfBounds {
                    edge,
                    count: 1,
                }),
            }
        }

        fn target(&self, edge: Edge) -> Result<Vertex, FreeCategoryError> {
            match edge.index() {
                0 => Ok(Vertex::new(1)),
                _ => Err(FreeCategoryError::EdgeOutOfBounds {
                    edge,
                    count: 1,
                }),
            }
        }
    }

    /// A triangle graph: three vertices, three edges forming a cycle.
    ///
    /// ```text
    ///   0 --e0--> 1 --e1--> 2 --e2--> 0
    /// ```
    struct TriangleGraph;

    impl Graph for TriangleGraph {
        fn vertex_count(&self) -> usize {
            3
        }

        fn edge_count(&self) -> usize {
            3
        }

        fn source(&self, edge: Edge) -> Result<Vertex, FreeCategoryError> {
            match edge.index() {
                0 => Ok(Vertex::new(0)),
                1 => Ok(Vertex::new(1)),
                2 => Ok(Vertex::new(2)),
                _ => Err(FreeCategoryError::EdgeOutOfBounds {
                    edge,
                    count: 3,
                }),
            }
        }

        fn target(&self, edge: Edge) -> Result<Vertex, FreeCategoryError> {
            match edge.index() {
                0 => Ok(Vertex::new(1)),
                1 => Ok(Vertex::new(2)),
                2 => Ok(Vertex::new(0)),
                _ => Err(FreeCategoryError::EdgeOutOfBounds {
                    edge,
                    count: 3,
                }),
            }
        }
    }

    #[test]
    fn identity_path_has_zero_length() {
        let p = Path::identity(Vertex::new(0));
        assert!(p.is_identity());
        assert_eq!(p.len(), 0);
        assert_eq!(p.source(), p.target());
    }

    #[test]
    fn singleton_path_has_correct_endpoints() -> Result<(), FreeCategoryError> {
        let graph = TwoVertexGraph;
        let p = Path::singleton(&graph, Edge::new(0))?;
        assert_eq!(p.source(), Vertex::new(0));
        assert_eq!(p.target(), Vertex::new(1));
        assert_eq!(p.len(), 1);
        Ok(())
    }

    #[test]
    fn singleton_out_of_bounds_returns_error() {
        let graph = TwoVertexGraph;
        let result = Path::singleton(&graph, Edge::new(5));
        assert!(result.is_err());
    }

    #[test]
    fn compose_matching_paths_succeeds() -> Result<(), FreeCategoryError> {
        let graph = TriangleGraph;
        let p1 = Path::singleton(&graph, Edge::new(0))?; // 0 -> 1
        let p2 = Path::singleton(&graph, Edge::new(1))?; // 1 -> 2
        let composed = p1.compose(p2)?;
        assert_eq!(composed.source(), Vertex::new(0));
        assert_eq!(composed.target(), Vertex::new(2));
        assert_eq!(composed.len(), 2);
        Ok(())
    }

    #[test]
    fn compose_mismatched_paths_fails() -> Result<(), FreeCategoryError> {
        let graph = TriangleGraph;
        let p1 = Path::singleton(&graph, Edge::new(0))?; // 0 -> 1
        let p2 = Path::singleton(&graph, Edge::new(2))?; // 2 -> 0
        let result = p1.compose(p2);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn identity_is_left_unit() -> Result<(), FreeCategoryError> {
        let graph = TwoVertexGraph;
        let id = Path::identity(Vertex::new(0));
        let p = Path::singleton(&graph, Edge::new(0))?; // 0 -> 1
        let composed = id.compose(p)?;
        assert_eq!(composed.source(), Vertex::new(0));
        assert_eq!(composed.target(), Vertex::new(1));
        assert_eq!(composed.len(), 1);
        Ok(())
    }

    #[test]
    fn identity_is_right_unit() -> Result<(), FreeCategoryError> {
        let graph = TwoVertexGraph;
        let p = Path::singleton(&graph, Edge::new(0))?; // 0 -> 1
        let id = Path::identity(Vertex::new(1));
        let composed = p.compose(id)?;
        assert_eq!(composed.source(), Vertex::new(0));
        assert_eq!(composed.target(), Vertex::new(1));
        assert_eq!(composed.len(), 1);
        Ok(())
    }

    #[test]
    fn composition_is_associative() -> Result<(), FreeCategoryError> {
        let graph = TriangleGraph;
        let e0 = Path::singleton(&graph, Edge::new(0))?; // 0 -> 1
        let e1 = Path::singleton(&graph, Edge::new(1))?; // 1 -> 2
        let e2 = Path::singleton(&graph, Edge::new(2))?; // 2 -> 0

        // (e0 ; e1) ; e2
        let e0_copy = Path::singleton(&graph, Edge::new(0))?;
        let e1_copy = Path::singleton(&graph, Edge::new(1))?;
        let e2_copy = Path::singleton(&graph, Edge::new(2))?;
        let left = e0.compose(e1)?.compose(e2)?;

        // e0 ; (e1 ; e2)
        let right = e0_copy.compose(e1_copy.compose(e2_copy)?)?;

        assert_eq!(left.source(), right.source());
        assert_eq!(left.target(), right.target());
        assert_eq!(left.len(), right.len());
        assert_eq!(left.edges(), right.edges());
        Ok(())
    }

    /// A trivial interpretation: count the number of edges in a path.
    struct CountMorphism;

    impl GraphMorphism<TriangleGraph> for CountMorphism {
        type Object = Vertex;
        type Morphism = usize;

        fn map_vertex(&self, v: Vertex) -> Vertex {
            v
        }

        fn map_edge(&self, _e: Edge) -> usize {
            1
        }
    }

    #[test]
    fn interpret_identity_gives_zero() {
        let id = Path::identity(Vertex::new(0));
        let result = interpret::<TriangleGraph, _>(
            &CountMorphism,
            &id,
            |_| 0,
            |a, b| a + b,
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn interpret_path_counts_edges() -> Result<(), FreeCategoryError> {
        let graph = TriangleGraph;
        let p = Path::singleton(&graph, Edge::new(0))?
            .compose(Path::singleton(&graph, Edge::new(1))?)?
            .compose(Path::singleton(&graph, Edge::new(2))?)?;

        let result = interpret::<TriangleGraph, _>(
            &CountMorphism,
            &p,
            |_| 0,
            |a, b| a + b,
        );
        assert_eq!(result, 3);
        Ok(())
    }
}
