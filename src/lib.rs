//! # comp-cat-rs
//!
//! Computational category theory in Rust: all constructions as Kan extensions.
//!
//! This crate is the Rust implementation of the thesis formalized in
//! `comp-cat-theory` (Lean 4): every constructive computation in category
//! theory is a Kan extension, and every verification is a decision problem.
//!
//! ## Architecture
//!
//! ```text
//! foundation/     Traits: Category, Functor, NatTrans, MonoidalCategory, Braided, Symmetric
//! primitive/      The ONE primitive: Kan extensions
//! collapse/       Every classical concept derived from primitive
//!                 Free categories, pullbacks, spans
//! effect/         Cats-Effect/ZIO-style runtime built on collapse
//! ```
//!
//! The `effect` layer provides a practical effect system whose design is
//! *justified* by the categorical collapses underneath.  IO, Resource,
//! Fiber, and Stream are all specific Kan extensions.
//!
//! The `foundation` monoidal hierarchy and `collapse` free category /
//! span / pullback modules provide the infrastructure for building
//! compositional ZK proof systems (e.g. halo2-style circuit categories)
//! on top of this crate.

pub mod foundation;
pub mod primitive;
pub mod collapse;
pub mod effect;
