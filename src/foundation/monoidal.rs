//! Monoidal categories: tensor product and coherence.
//!
//! A monoidal category is a category equipped with a bifunctor
//! `⊗` (tensor product) and a unit object `I`, together with natural
//! isomorphisms (associator, left unitor, right unitor) satisfying
//! coherence conditions (pentagon and triangle identities).
//!
//! In the halo2 context:
//! - Objects = circuit wire types (field elements, booleans, etc.)
//! - Morphisms = circuit gates / gadgets
//! - Tensor product = parallel composition ("run sub-circuits side by side")
//! - Unit = the empty circuit
//! - Braiding = wire permutation (PLONK permutation argument)

use super::category::Category;
use super::iso::Iso;

/// A monoidal category: [`Category`] + tensor product + unit + coherence.
///
/// # Laws (coherence, from Mac Lane; verified in Lean 4)
///
/// Pentagon identity and triangle identity hold for
/// the associator and unitors.
pub trait MonoidalCategory: Category {
    /// The tensor product of two objects.
    ///
    /// In halo2: parallel composition of wire bundles.
    type Tensor<A: Into<Self>, B: Into<Self>>: Into<Self>;

    /// The unit object.
    ///
    /// In halo2: the empty wire bundle.
    type Unit: Into<Self> + Clone;

    /// The bifunctor action on morphisms: given `f: A -> B` and
    /// `g: C -> D`, produce `f ⊗ g: A ⊗ C -> B ⊗ D`.
    ///
    /// In halo2: parallel composition of gates / gadgets.
    fn tensor_map<A, B, C, D>(
        f: Self::Hom<A, B>,
        g: Self::Hom<C, D>,
    ) -> Self::Hom<Self::Tensor<A, C>, Self::Tensor<B, D>>
    where
        A: Into<Self>,
        B: Into<Self>,
        C: Into<Self>,
        D: Into<Self>;

    /// Associator: `(A ⊗ B) ⊗ C ≅ A ⊗ (B ⊗ C)`.
    #[allow(clippy::type_complexity)]
    fn associator<A, B, C>() -> Iso<
        Self,
        Self::Tensor<Self::Tensor<A, B>, C>,
        Self::Tensor<A, Self::Tensor<B, C>>,
    >
    where
        A: Into<Self>,
        B: Into<Self>,
        C: Into<Self>;

    /// Left unitor: `I ⊗ A ≅ A`.
    fn left_unitor<A>() -> Iso<Self, Self::Tensor<Self::Unit, A>, A>
    where
        A: Into<Self>;

    /// Right unitor: `A ⊗ I ≅ A`.
    fn right_unitor<A>() -> Iso<Self, Self::Tensor<A, Self::Unit>, A>
    where
        A: Into<Self>;
}

/// A braided monoidal category: [`MonoidalCategory`] + braiding.
///
/// The braiding `σ_{A,B}: A ⊗ B ≅ B ⊗ A` satisfies hexagon identities.
///
/// In halo2: wire permutation between parallel sub-circuits.
pub trait Braided: MonoidalCategory {
    /// The braiding: `A ⊗ B ≅ B ⊗ A`.
    fn braid<A, B>() -> Iso<Self, Self::Tensor<A, B>, Self::Tensor<B, A>>
    where
        A: Into<Self>,
        B: Into<Self>;
}

/// A symmetric monoidal category: [`Braided`] + involutivity.
///
/// Additional law: `σ_{B,A} ∘ σ_{A,B} = id_{A⊗B}`
///
/// This is a marker trait; the involutivity law is documented
/// but not enforced (verified in Lean 4).
///
/// In halo2: the wire permutation is its own inverse.
pub trait Symmetric: Braided {}
