//! Effect: the practical effect system built on categorical foundations.
//!
//! This is the Cats-Effect/ZIO equivalent.  Each type here is a
//! specific instantiation of the categorical machinery:
//!
//! - `Io<E, A>`: an effectful computation (a specific monad)
//! - `Resource<E, A>`: bracket-based resource management
//! - `Stream<E, A>`: effectful iteration (colimit, hence Lan)
//! - `Fiber<E, A>`: lightweight concurrency (fork/join)
//!
//! Categorically:
//! - `Io` is a free monad over a set of primitive effects,
//!   which is a left adjoint (hence a Kan extension).
//! - `Resource` uses the bracket pattern, which is an adjunction
//!   between acquisition and release (hence two Kan extensions).
//! - `Stream` is a colimit (iterative construction), hence a Lan.
//! - `Fiber::fork` is a coproduct, `Fiber::join` is a limit.

pub mod io;
pub mod resource;
pub mod stream;
pub mod fiber;
pub mod instances;
