//! The Category trait: objects, morphisms, composition, identity.
//!
//! In Rust, a "category" is a trait on the object type, with
//! morphisms as a GAT (generic associated type) indexed by
//! source and target objects.
//!
//! The laws (associativity, unit) are documented but not enforced
//! at the type level -- Rust's type system cannot express dependent
//! equalities.  The Lean 4 formalization provides the proofs;
//! this crate provides the runtime implementation.

/// A category with objects of type `Self` and morphisms between them.
///
/// # Laws (enforced by the Lean 4 spec, trusted here)
///
/// - `comp(f, id) == f`           (right unit)
/// - `comp(id, f) == f`           (left unit)
/// - `comp(comp(f, g), h) == comp(f, comp(g, h))`  (associativity)
pub trait Category: Sized {
    /// The type of morphisms from `src` to `tgt`.
    type Hom<A: Into<Self>, B: Into<Self>>;

    /// The identity morphism on an object.
    fn id<A: Into<Self> + Clone>(a: &A) -> Self::Hom<A, A>;

    /// Composition: `f: A -> B` and `g: B -> C` yield `g . f: A -> C`.
    fn comp<A, B, C>(
        f: Self::Hom<A, B>,
        g: Self::Hom<B, C>,
    ) -> Self::Hom<A, C>
    where
        A: Into<Self>,
        B: Into<Self>,
        C: Into<Self>;
}
