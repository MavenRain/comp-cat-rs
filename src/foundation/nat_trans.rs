//! Natural transformations between functors.
//!
//! A natural transformation `α: F ⇒ G` provides, for every type `A`,
//! a morphism `α_A: F<A> -> G<A>`, such that for any `f: A -> B`:
//!
//! ```text
//! G::map(α_A(fa), f) == α_B(F::map(fa, f))   (naturality square)
//! ```
//!
//! In the halo2 context, natural transformations express backend
//! polymorphism: a circuit description in one polynomial commitment
//! scheme (IPA) can be naturally transformed to another (KZG).
//!
//! ## Design note
//!
//! `NatTrans` is not object-safe because `transform` is generic
//! over `A`.  For runtime backend selection, use enum dispatch
//! (matching the crate's existing `Io` enum pattern) rather than
//! trait objects.
//!
//! ## Laws (naturality, enforced by the Lean 4 spec)
//!
//! For all `f: A -> B`:
//! ```text
//! G::map(transform::<A>(fa), f) == transform::<B>(F::map(fa, f))
//! ```

use super::kind::Kind;

/// A natural transformation from functor `Source` to functor `Target`.
///
/// Implemented on a zero-sized witness type, enabling static dispatch
/// and monomorphization.
pub trait NatTrans {
    /// The source functor.
    type Source: Kind;

    /// The target functor.
    type Target: Kind;

    /// The component at type `A`: `Source<A> -> Target<A>`.
    fn transform<A>(
        fa: <Self::Source as Kind>::F<A>,
    ) -> <Self::Target as Kind>::F<A>;
}
