//! The Monad trait: sequencing effectful computations.
//!
//! Categorically, a monad is a monoid in the category of
//! endofunctors.  In the Cats Effect/ZIO sense, it provides
//! `pure` (unit) and `flat_map` (bind/Kleisli composition).
//!
//! By Collapse/Adjunction in the Lean 4 spec, every monad
//! arises from an adjunction, and every adjunction is a pair
//! of Kan extensions.  So every monad is, at bottom, a Kan
//! extension.
//!
//! ## Laws (from the Lean 4 spec)
//!
//! - Left unit:  `flat_map(pure(a), f) == f(a)`
//! - Right unit: `flat_map(fa, pure) == fa`
//! - Associativity: `flat_map(flat_map(fa, f), g) == flat_map(fa, |a| flat_map(f(a), g))`

use super::functor::Functor;

/// A monad: a functor with `pure` and `flat_map`.
///
/// `Functor::map` is derivable from `pure` + `flat_map`:
/// `map(fa, f) = flat_map(fa, |a| pure(f(a)))`
///
/// Implementations should ensure this derived `map` agrees
/// with the `Functor` impl.
pub trait Monad: Functor {
    /// Lift a pure value into the monadic context.
    ///
    /// Categorically: the unit `η : Id -> T` of the monad.
    fn pure<A: Send + 'static>(a: A) -> Self::F<A>;

    /// Sequence two computations, feeding the result of the
    /// first into a function that produces the second.
    ///
    /// Categorically: the Kleisli composition, derived from
    /// the multiplication `μ : T² -> T` of the monad.
    fn flat_map<A: Send + 'static, B: Send + 'static>(fa: Self::F<A>, f: impl FnOnce(A) -> Self::F<B> + Send + 'static) -> Self::F<B>;
}
