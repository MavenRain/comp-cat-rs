//! Higher-kinded type encoding via GATs.
//!
//! Rust doesn't have native HKT (`F[_]` in Scala).  We encode
//! type constructors using a witness type + a generic associated type.
//!
//! A "kind" is a type-level function `* -> *`.  For example,
//! `Option` is a kind that maps `A` to `Option<A>`.  In Rust,
//! we represent this as:
//!
//! ```rust
//! struct OptionK;  // witness type (zero-sized)
//!
//! impl Kind for OptionK {
//!     type F<A> = Option<A>;
//! }
//! ```
//!
//! Categorically, `Kind` is an endofunctor on the category of
//! Rust types (the action on objects).  The action on morphisms
//! is provided by `Functor::map`.

/// A type constructor: maps types to types.
///
/// Implementing this trait for a witness type `W` declares
/// that `W` represents the type constructor `A |-> W::F<A>`.
pub trait Kind {
    /// Apply the type constructor to a concrete type.
    type F<A>;
}
