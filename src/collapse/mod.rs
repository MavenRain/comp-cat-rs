//! Collapse: every classical concept is a Kan extension.
//!
//! This module documents the collapse hierarchy from the Lean 4
//! formalization.  Each submodule corresponds to one Collapse/
//! file in the spec and explains how the Rust types in `effect/`
//! are categorical constructions.
//!
//! ```text
//! Monad         -> Adjunction -> Kan Extension
//! IO            -> Free Monad -> Free/Forgetful Adjunction -> Kan
//! Resource      -> Bracket    -> Adjunction -> Kan
//! Fiber         -> Free Monad -> Kan
//! Stream        -> Colimit    -> Left Kan Extension
//! Free Category -> Left Adjoint (Free ⊣ Forgetful) -> Left Kan Extension
//! Pullback      -> Limit      -> Right Kan Extension
//! Span Comp     -> Pullback   -> Right Kan Extension
//! ```
//!
//! In the Lean 4 spec, each collapse is a fully proved theorem.
//! Here, the collapse is a design justification: the Rust types
//! are *implementations* of the categorical constructions.

pub mod adjunction;
pub mod monad_is_kan;
pub mod free_category;
pub mod limit;
pub mod span;
