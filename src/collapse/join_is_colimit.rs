//! The collapse: joins in posetal categories are colimits (left Kan extensions).
//!
//! This module documents the chain:
//!
//! ```text
//! join(a, b) in a join-semilattice
//!   = coproduct of a and b in the posetal category  [semilattice = poset with coproducts]
//!   = colimit of the discrete diagram {a, b}        [binary coproduct = colimit over 2]
//!   = left Kan extension of the diagram             [colimits are left Kan extensions]
//! ```
//!
//! More precisely: let `J = {0, 1}` be the discrete category (two
//! objects, only identity morphisms).  Let `D: J -> Poset` be the
//! diagram sending `0 |-> a` and `1 |-> b`.  Let `!: J -> 1` be
//! the unique functor to the terminal category.
//!
//! Then `join(a, b) = (Lan_! D)(*)`, the left Kan extension of `D`
//! along `!`, evaluated at the unique object `*` of `1`.
//!
//! In Rust, every `impl JoinSemilattice for T` is an implementation
//! of this chain.  The `join` method computes the colimit.  The
//! correctness (commutativity, associativity, idempotency) is the
//! statement that the colimit cocone is universal.
//!
//! ## Duality with limits
//!
//! The dual construction (meet-semilattice) gives limits in posetal
//! categories.  Meets are products, which are right Kan extensions.
//! This is dual to `collapse::limit`, which provides pullbacks
//! (limits over cospan diagrams) in general categories.
//!
//! No runtime code here; this module exists for the architecture.
