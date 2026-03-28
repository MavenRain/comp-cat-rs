//! Isomorphisms in a category.
//!
//! An isomorphism between objects `A` and `B` in a category `C`
//! consists of a morphism `forward: A -> B` and an inverse
//! `backward: B -> A` such that their compositions are identities.
//!
//! Needed for coherence isomorphisms in monoidal categories
//! (associator, left/right unitors, braiding) and for expressing
//! equivalences between circuit representations.

use super::category::Category;

/// An isomorphism in category `C` between objects `A` and `B`.
///
/// # Laws (trusted from the Lean 4 spec)
///
/// - `comp(forward, backward) == id_A`
/// - `comp(backward, forward) == id_B`
#[must_use]
pub struct Iso<C, A, B>
where
    C: Category,
    A: Into<C>,
    B: Into<C>,
{
    forward: C::Hom<A, B>,
    backward: C::Hom<B, A>,
}

impl<C, A, B> Iso<C, A, B>
where
    C: Category,
    A: Into<C>,
    B: Into<C>,
{
    /// Construct an isomorphism from a forward and backward morphism.
    ///
    /// The caller asserts the round-trip laws hold.  This is verified
    /// in the Lean 4 formalization, not at runtime.
    pub fn new(forward: C::Hom<A, B>, backward: C::Hom<B, A>) -> Self {
        Self { forward, backward }
    }

    /// The forward morphism: `A -> B`.
    pub fn forward(&self) -> &C::Hom<A, B> {
        &self.forward
    }

    /// The backward morphism: `B -> A`.
    pub fn backward(&self) -> &C::Hom<B, A> {
        &self.backward
    }

    /// Consume this isomorphism, returning the forward morphism.
    pub fn into_forward(self) -> C::Hom<A, B> {
        self.forward
    }

    /// Consume this isomorphism, returning the backward morphism.
    pub fn into_backward(self) -> C::Hom<B, A> {
        self.backward
    }

    /// Flip the isomorphism: produces `Iso<C, B, A>`.
    pub fn flip(self) -> Iso<C, B, A> {
        Iso {
            forward: self.backward,
            backward: self.forward,
        }
    }
}
