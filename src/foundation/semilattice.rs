//! Join-semilattices: partial orders with least upper bounds.
//!
//! A join-semilattice is a partial order where every pair of elements
//! has a least upper bound (join).  Categorically, this is a thin
//! category (posetal category) with finite coproducts.
//!
//! The join is the binary coproduct: `join(a, b)` is the universal
//! object such that `a <= join(a, b)` and `b <= join(a, b)`, and
//! for any `c` with `a <= c` and `b <= c`, `join(a, b) <= c`.
//!
//! By `collapse::join_is_colimit`, every join is a left Kan
//! extension.
//!
//! ## Laws (enforced by the Lean 4 spec, trusted here)
//!
//! - Commutativity:  `join(a, b) == join(b, a)`
//! - Associativity:  `join(join(a, b), c) == join(a, join(b, c))`
//! - Idempotency:    `join(a, a) == a`
//! - Upper bound:    `a <= join(a, b)` and `b <= join(a, b)`
//! - Least:          if `a <= c` and `b <= c`, then `join(a, b) <= c`

/// A join-semilattice: a partial order with binary least upper bounds.
///
/// Implementing this trait asserts that the type's [`PartialOrd`]
/// relation forms a partial order and that `join` computes the
/// least upper bound.
pub trait JoinSemilattice: PartialOrd + Eq {
    /// The least upper bound of two elements.
    ///
    /// Categorically: the coproduct in the posetal category,
    /// which is a colimit, which is a left Kan extension.
    #[must_use]
    fn join(&self, other: &Self) -> Self;
}
