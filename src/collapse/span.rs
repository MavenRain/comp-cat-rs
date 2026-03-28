//! Spans: the categorical structure of lookup arguments.
//!
//! A span from `A` to `B` is a pair of morphisms with common domain:
//!
//! ```text
//!     A <--left-- S --right--> B
//! ```
//!
//! Spans compose via pullback: the composition of `(S: A -> B)`
//! and `(T: B -> C)` has apex = pullback of `S.right` and `T.left`.
//!
//! In halo2:
//! - A span models a lookup argument: `S` is the lookup table,
//!   `left` projects to the input columns, `right` projects to the
//!   output columns
//! - Span composition models chained lookups
//! - The pullback in the composition is the "shared witness"

use super::free_category::FreeCategoryError;
use super::limit::{Cospan, HasPullbacks, Pullback};

/// A span: two morphisms with a common domain.
///
/// ```text
///     A <--left-- S --right--> B
/// ```
///
/// In halo2: `S` is the lookup table, `left` maps to input
/// columns, `right` maps to output columns.
#[must_use]
pub struct Span<Obj, Mor> {
    apex: Obj,
    left: Mor,
    right: Mor,
}

impl<Obj, Mor> Span<Obj, Mor> {
    /// Construct a span from an apex and two legs.
    pub fn new(apex: Obj, left: Mor, right: Mor) -> Self {
        Self { apex, left, right }
    }

    /// The apex (common domain) of the span.
    #[must_use]
    pub fn apex(&self) -> &Obj {
        &self.apex
    }

    /// The left leg: `S -> A`.
    #[must_use]
    pub fn left(&self) -> &Mor {
        &self.left
    }

    /// The right leg: `S -> B`.
    #[must_use]
    pub fn right(&self) -> &Mor {
        &self.right
    }
}

/// The result of composing two spans via pullback.
///
/// Given `Span(A <-f- S -g-> B)` and `Span(B <-h- T -k-> C)`,
/// the composition is `Span(A <-f∘p1- P -k∘p2-> C)` where
/// `P` is the pullback of `g` and `h`.
#[must_use]
pub struct SpanComposition<Obj, Mor> {
    composed: Span<Obj, Mor>,
    pullback: Pullback<Obj, Mor>,
}

impl<Obj, Mor> SpanComposition<Obj, Mor> {
    /// The composed span.
    pub fn span(&self) -> &Span<Obj, Mor> {
        &self.composed
    }

    /// The pullback used in the composition (the shared witness).
    pub fn pullback(&self) -> &Pullback<Obj, Mor> {
        &self.pullback
    }
}

/// Compose two spans via pullback.
///
/// The right leg of `first` and the left leg of `second` form a
/// cospan over the shared object `B`.  The pullback of this cospan
/// produces the apex of the composed span.
///
/// ```text
///           first                second
///     A <--fl-- S --fr--> B <--sl-- T --sr--> C
///
///     Composed via pullback of (fr, sl):
///
///     A <--fl∘p1-- P --sr∘p2--> C
/// ```
///
/// # Errors
///
/// Returns an error if the pullback cannot be computed
/// (e.g., the cospan formed by the inner legs is ill-formed).
pub fn compose_spans<C>(
    first: Span<C::Obj, C::Mor>,
    second: Span<C::Obj, C::Mor>,
    shared_apex: C::Obj,
    category: &C,
    comp_fn: impl Fn(C::Mor, C::Mor) -> C::Mor,
) -> Result<SpanComposition<C::Obj, C::Mor>, FreeCategoryError>
where
    C: HasPullbacks,
    C::Obj: Clone,
    C::Mor: Clone,
{
    let cospan = Cospan::new(first.right, second.left, shared_apex);
    let pullback = category.pullback(cospan)?;

    let composed_left = comp_fn(pullback.proj_left().clone(), first.left);
    let composed_right = comp_fn(pullback.proj_right().clone(), second.right);

    let composed = Span::new(
        pullback.apex().clone(),
        composed_left,
        composed_right,
    );

    Ok(SpanComposition { composed, pullback })
}
