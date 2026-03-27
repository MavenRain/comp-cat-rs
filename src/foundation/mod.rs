//! Foundation: the core trait hierarchy.
//!
//! Encodes higher-kinded types in Rust via GATs, then builds
//! Functor, Applicative, Monad on top.  These are the practical
//! programming interface; the categorical justification lives
//! in `primitive/` and `collapse/`.

pub mod kind;
pub mod functor;
pub mod monad;

pub use kind::Kind;
pub use functor::Functor;
pub use monad::Monad;
