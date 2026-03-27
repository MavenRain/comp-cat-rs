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
//! foundation/     Traits: Category, Functor, NatTrans
//! primitive/      The ONE primitive: Kan extensions
//! collapse/       Every classical concept derived from primitive
//! effect/         Cats-Effect/ZIO-style runtime built on collapse
//! ```
//!
//! The `effect` layer is why this crate exists: it provides a practical
//! effect system whose design is *justified* by the categorical collapses
//! underneath.  IO, Resource, Fiber, and Stream are all specific Kan
//! extensions.

pub mod foundation;
pub mod primitive;
pub mod collapse;
pub mod effect;
