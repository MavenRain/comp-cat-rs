//! Foundation: the core trait hierarchy.
//!
//! Encodes higher-kinded types in Rust via GATs, then builds
//! Category, Functor, `NatTrans`, and monoidal structure on top.
//! These are the practical programming interface; the categorical
//! justification lives in `primitive/` and `collapse/`.

pub mod kind;
pub mod functor;
pub mod monad;
pub mod category;
pub mod iso;
pub mod nat_trans;
pub mod monoidal;

pub use kind::Kind;
pub use functor::Functor;
pub use monad::Monad;
pub use category::Category;
pub use iso::Iso;
pub use nat_trans::NatTrans;
pub use monoidal::{MonoidalCategory, Braided, Symmetric};
