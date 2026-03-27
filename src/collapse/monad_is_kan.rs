//! The collapse: every monad is a Kan extension.
//!
//! This module documents the chain:
//!
//! ```text
//! Monad T on C
//!   = Eilenberg-Moore adjunction F^T ⊣ U^T
//!   = (Lan_{F^T}(Id), Ran_{U^T}(Id))     [by Collapse/Adjunction]
//!   = pair of Kan extensions               [by Collapse/MonadAlgebra]
//! ```
//!
//! In Rust, every `impl Monad for SomeK` is an implementation of
//! this chain.  The `pure` is the adjunction unit (Lan unit), and
//! `flat_map` is composition in the Kleisli category.
//!
//! No runtime code here; this module exists for the architecture.
