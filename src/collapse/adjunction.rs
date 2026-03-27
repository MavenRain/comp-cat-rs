//! Adjunctions in Rust: the categorical backbone of the effect system.
//!
//! An adjunction F ⊣ G relates two functors via natural bijections.
//! In the Lean 4 spec, we proved:
//!   - G = `Lan_F(Id)`  (right adjoint is a left Kan extension)
//!   - F = `Ran_G(Id)`  (left adjoint is a right Kan extension)
//!
//! In Rust, adjunctions manifest as:
//!   - `Monad::pure` + `Monad::flat_map` (the Kleisli adjunction)
//!   - Resource acquire/release (bracket as adjunction)
//!   - Free/forgetful pairs (free monads)
//!
//! This module provides the `Adjunction` trait that documents
//! this structure.

use crate::foundation::{Functor, Kind};

/// An adjunction between two functors: `Left ⊣ Right`.
///
/// The unit (`pure`) and counit (`extract`) witness the adjunction.
/// In practice, `Monad` is the Kleisli-category formulation of
/// the adjunction `Free ⊣ Forgetful`.
pub trait Adjunction {
    /// The left adjoint (e.g., the free functor).
    type Left: Functor;

    /// The right adjoint (e.g., the forgetful functor).
    type Right: Functor;

    /// The unit: `A -> Right(Left(A))`.
    fn unit<A>(
        a: A,
    ) -> <Self::Right as Kind>::F<<Self::Left as Kind>::F<A>>;

    /// The counit: `Left(Right(A)) -> A`.
    fn counit<A>(
        fga: <Self::Left as Kind>::F<<Self::Right as Kind>::F<A>>,
    ) -> A;
}
