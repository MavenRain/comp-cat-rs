//! Pullbacks: limits over the cospan diagram.
//!
//! A pullback of a cospan `A -f-> C <-g- B` is an object `P` with
//! projections `p1: P -> A` and `p2: P -> B` such that `f ∘ p1 = g ∘ p2`,
//! universal among all such cones.
//!
//! As a right Kan extension: the pullback is `Ran` along the
//! cospan inclusion into the arrow category.
//!
//! In halo2:
//! - Copy constraints: two wires equated via pullback over the
//!   "equality" cospan
//! - Lookup sharing: two sub-circuits sharing a lookup table
//!   via pullback over the table inclusion

use super::free_category::FreeCategoryError;

/// A cospan: two morphisms with a common codomain.
///
/// ```text
///     A --left--> C <--right-- B
/// ```
///
/// In halo2: the common codomain `C` is a lookup table,
/// and `left`, `right` are the lookup columns in two sub-circuits.
#[must_use]
pub struct Cospan<Obj, Mor> {
    left: Mor,
    right: Mor,
    apex: Obj,
}

impl<Obj, Mor> Cospan<Obj, Mor> {
    /// Construct a cospan from two morphisms and their common codomain.
    pub fn new(left: Mor, right: Mor, apex: Obj) -> Self {
        Self { left, right, apex }
    }

    /// The left leg of the cospan: `A -> C`.
    #[must_use]
    pub fn left(&self) -> &Mor {
        &self.left
    }

    /// The right leg of the cospan: `B -> C`.
    #[must_use]
    pub fn right(&self) -> &Mor {
        &self.right
    }

    /// The apex (common codomain) of the cospan.
    #[must_use]
    pub fn apex(&self) -> &Obj {
        &self.apex
    }
}

/// A pullback: the limit of a cospan.
///
/// ```text
///     P --proj_left--> A
///     |                |
///   proj_right       left
///     |                |
///     v                v
///     B ---right-----> C
/// ```
///
/// The square commutes: `left ∘ proj_left = right ∘ proj_right`.
#[must_use]
pub struct Pullback<Obj, Mor> {
    apex: Obj,
    proj_left: Mor,
    proj_right: Mor,
    cospan: Cospan<Obj, Mor>,
}

impl<Obj, Mor> Pullback<Obj, Mor> {
    /// Construct a pullback from its components.
    ///
    /// The caller asserts the commutativity condition and
    /// universal property hold (verified in Lean 4).
    pub fn new(
        apex: Obj,
        proj_left: Mor,
        proj_right: Mor,
        cospan: Cospan<Obj, Mor>,
    ) -> Self {
        Self {
            apex,
            proj_left,
            proj_right,
            cospan,
        }
    }

    /// The pullback object.
    #[must_use]
    pub fn apex(&self) -> &Obj {
        &self.apex
    }

    /// The left projection: `P -> A`.
    #[must_use]
    pub fn proj_left(&self) -> &Mor {
        &self.proj_left
    }

    /// The right projection: `P -> B`.
    #[must_use]
    pub fn proj_right(&self) -> &Mor {
        &self.proj_right
    }

    /// The cospan that this is a pullback of.
    pub fn cospan(&self) -> &Cospan<Obj, Mor> {
        &self.cospan
    }
}

/// A category that has all pullbacks.
///
/// In halo2: the circuit category has pullbacks because
/// copy constraints and lookup sharing always exist.
pub trait HasPullbacks {
    /// The object type in this category.
    type Obj;

    /// The morphism type in this category.
    type Mor;

    /// Compute the pullback of a cospan.
    ///
    /// # Errors
    ///
    /// Returns an error if the pullback cannot be constructed
    /// (e.g., the cospan is ill-formed).
    fn pullback(
        &self,
        cospan: Cospan<Self::Obj, Self::Mor>,
    ) -> Result<Pullback<Self::Obj, Self::Mor>, FreeCategoryError>;
}
