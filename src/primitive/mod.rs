//! Primitive: the Kan extension, expressed as a Rust type.
//!
//! This module exists to document the thesis, not to provide
//! runtime computation.  The Lean 4 formalization proves that
//! every construction in `collapse/` is a Kan extension; this
//! module makes that connection visible in the Rust crate.
//!
//! In practice, you use the `foundation/` traits (Functor, Monad)
//! and the `effect/` types (Io, Resource, Fiber).  The Kan
//! extension is the *reason* those abstractions are correct.

pub mod kan;
