//! Kan extensions: the single universal construction.
//!
//! Every construction in computational category theory -- limits,
//! colimits, adjunctions, exponentials, ends, free algebras,
//! monad algebras, factorizations, subobject classifiers -- is
//! a Kan extension.  This is the thesis of the `comp-cat-theory`
//! Lean 4 formalization.
//!
//! In Rust, we express Kan extensions as types parameterized by
//! the functors involved.  The `collapse/` module then shows
//! how each classical concept instantiates this type.
//!
//! # Right Kan Extension
//!
//! Given functors `K: J -> C` and `F: J -> D`, the right Kan
//! extension `Ran_K(F): C -> D` is the "best approximation" of
//! `F` along `K`, equipped with a counit and a universal property.
//!
//! # Left Kan Extension
//!
//! Dual: the "best co-approximation" with a unit.
//!
//! # Connection to the effect layer
//!
//! - `Monad::pure` is the unit of a left Kan extension
//! - `Monad::flat_map` is composition in the Kleisli category
//! - `Io` is a specific monad arising from a free/forgetful adjunction
//! - `Resource` is a specific use of the bracket pattern (an adjunction)
//! - Every adjunction decomposes into two Kan extensions (see `collapse/`)

use crate::foundation::{Functor, Kind};

/// A right Kan extension of `F` along `K`.
///
/// This is the type-level statement that `ran_functor` is the
/// right Kan extension, equipped with a counit witness.
///
/// In practice, you rarely construct this directly.  The
/// `collapse/` module provides constructors from limits,
/// adjunctions, etc.
pub struct RightKan<K: Kind, F: Kind, Ran: Kind> {
    _k: core::marker::PhantomData<K>,
    _f: core::marker::PhantomData<F>,
    _ran: core::marker::PhantomData<Ran>,
}

/// A left Kan extension of `F` along `K`.
///
/// Dual of `RightKan`.
pub struct LeftKan<K: Kind, F: Kind, Lan: Kind> {
    _k: core::marker::PhantomData<K>,
    _f: core::marker::PhantomData<F>,
    _lan: core::marker::PhantomData<Lan>,
}

/// Marker trait: `Ran` is a right Kan extension of `F` along `K`.
///
/// This is a compile-time assertion, not a runtime computation.
/// It documents the categorical justification for why a particular
/// type constructor exists.
pub trait IsRightKan<K: Kind, F: Kind>: Functor {}

/// Marker trait: `Lan` is a left Kan extension of `F` along `K`.
pub trait IsLeftKan<K: Kind, F: Kind>: Functor {}
