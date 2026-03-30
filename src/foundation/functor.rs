//! The Functor trait: structure-preserving maps between types.
//!
//! Categorically, a functor is a structure-preserving map between
//! categories.  In the Cats Effect/ZIO sense, a Functor is a
//! type constructor `F` equipped with `map: F<A> -> (A -> B) -> F<B>`.
//!
//! This is the action on morphisms of the endofunctor on the
//! category of Rust types.  The action on objects is `Kind::F`.
//!
//! ## Laws (from the Lean 4 spec)
//!
//! - Identity: `map(fa, |a| a) == fa`
//! - Composition: `map(map(fa, f), g) == map(fa, |a| g(f(a)))`

use super::kind::Kind;

/// A functor: a type constructor with a law-abiding `map`.
///
/// Extends `Kind` with the morphism action.
pub trait Functor: Kind {
    /// Apply a function inside the functor context.
    fn map<A: Send + 'static, B: Send + 'static>(fa: Self::F<A>, f: impl FnOnce(A) -> B + Send + 'static) -> Self::F<B>;
}
