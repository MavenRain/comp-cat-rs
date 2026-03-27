//! IO: the core effect type.
//!
//! `Io<E, A>` represents a lazy, composable, effectful computation
//! that either produces a value of type `A` or fails with an error
//! of type `E`.
//!
//! Unlike `Result<A, E>`, an `Io<E, A>` is not evaluated until
//! explicitly run.  This allows composition via `map`, `flat_map`,
//! etc. without executing side effects.
//!
//! Categorically, `Io` is a monad (see `foundation::monad`),
//! which by `collapse::monad_is_kan` is a pair of Kan extensions.

use crate::foundation::Kind;

/// Witness type for the IO kind, parameterized by error type.
///
/// `IoK<E>` represents the type constructor `A |-> Io<E, A>`.
pub struct IoK<E> {
    _phantom: core::marker::PhantomData<E>,
}

/// A lazy, composable, effectful computation.
///
/// Constructed via combinators (`pure`, `map`, `flat_map`, `suspend`),
/// never directly.  Executed only when explicitly `run`.
pub enum Io<E, A> {
    /// A pure value, already computed.
    Pure(A),

    /// A suspended side effect.
    Suspend(Box<dyn FnOnce() -> Result<A, E>>),

    /// A sequenced computation: run `source`, then feed to `cont`.
    FlatMap(Box<IoFlatMap<E, A>>),
}

/// Helper for the `FlatMap` variant (existential over intermediate type).
///
/// Rust doesn't have native existentials, so we erase the
/// intermediate type `B` behind a trait object.
pub(crate) trait IoStep<E, A> {
    fn run(self: Box<Self>) -> Result<A, E>;
}

/// Concrete `FlatMap` node: `source: Io<E, B>` then `f: B -> Io<E, A>`.
pub struct IoFlatMap<E, A> {
    inner: Box<dyn IoStep<E, A>>,
}

struct FlatMapImpl<E, B, A> {
    source: Io<E, B>,
    cont: Box<dyn FnOnce(B) -> Io<E, A>>,
}

impl<E, B, A> IoStep<E, A> for FlatMapImpl<E, B, A> {
    fn run(self: Box<Self>) -> Result<A, E> {
        let FlatMapImpl { source, cont } = *self;
        source.run().and_then(|b| cont(b).run())
    }
}

impl<E, A> Io<E, A> {
    /// Lift a pure value.
    #[must_use]
    pub fn pure(a: A) -> Self {
        Self::Pure(a)
    }

    /// Suspend a side-effecting computation.
    #[must_use]
    pub fn suspend(f: impl FnOnce() -> Result<A, E> + 'static) -> Self
    where
        E: 'static,
        A: 'static,
    {
        Self::Suspend(Box::new(f))
    }

    /// Map a function over the result.
    #[must_use]
    pub fn map<B>(self, f: impl FnOnce(A) -> B + 'static) -> Io<E, B>
    where
        E: 'static,
        A: 'static,
        B: 'static,
    {
        self.flat_map(move |a| Io::Pure(f(a)))
    }

    /// Sequence with a function that produces the next computation.
    #[must_use]
    pub fn flat_map<B>(self, f: impl FnOnce(A) -> Io<E, B> + 'static) -> Io<E, B>
    where
        E: 'static,
        A: 'static,
        B: 'static,
    {
        Io::FlatMap(Box::new(IoFlatMap {
            inner: Box::new(FlatMapImpl {
                source: self,
                cont: Box::new(f),
            }),
        }))
    }

    /// Capture errors into the success channel.
    ///
    /// The resulting `Io` never fails; the error is wrapped in the `Result`.
    #[must_use]
    pub fn attempt(self) -> Io<core::convert::Infallible, Result<A, E>>
    where
        E: 'static,
        A: 'static,
    {
        Io::Suspend(Box::new(move || Ok(self.run())))
    }

    /// Recover from errors with a pure handler.
    #[must_use]
    pub fn handle_error(self, handler: impl FnOnce(E) -> A + 'static) -> Io<E, A>
    where
        E: 'static,
        A: 'static,
    {
        Io::Suspend(Box::new(move || match self.run() {
            Ok(a) => Ok(a),
            Err(e) => Ok(handler(e)),
        }))
    }

    /// Recover from errors with an effectful handler.
    #[must_use]
    pub fn handle_error_with<E2: 'static>(
        self,
        handler: impl FnOnce(E) -> Io<E2, A> + 'static,
    ) -> Io<E2, A>
    where
        E: 'static,
        A: 'static,
    {
        Io::Suspend(Box::new(move || match self.run() {
            Ok(a) => Ok(a),
            Err(e) => handler(e).run(),
        }))
    }

    /// Transform the error type.
    #[must_use]
    pub fn map_error<E2: 'static>(self, f: impl FnOnce(E) -> E2 + 'static) -> Io<E2, A>
    where
        E: 'static,
        A: 'static,
    {
        Io::Suspend(Box::new(move || self.run().map_err(f)))
    }

    /// Sequentially combine two computations, collecting both results.
    #[must_use]
    pub fn zip<B: 'static>(self, other: Io<E, B>) -> Io<E, (A, B)>
    where
        E: 'static,
        A: 'static,
    {
        self.flat_map(move |a| other.map(move |b| (a, b)))
    }

    /// Discard the result, keeping only the effect.
    #[must_use]
    pub fn as_unit(self) -> Io<E, ()>
    where
        E: 'static,
        A: 'static,
    {
        self.map(|_| ())
    }

    /// Execute the computation, producing a `Result`.
    ///
    /// This is the only function that performs side effects.
    /// Everything else is pure composition.
    ///
    /// # Errors
    ///
    /// Returns `Err(E)` if any suspended effect in the computation
    /// chain fails with an error of type `E`.
    pub fn run(self) -> Result<A, E> {
        match self {
            Self::Pure(a) => Ok(a),
            Self::Suspend(f) => f(),
            Self::FlatMap(fm) => fm.inner.run(),
        }
    }
}

impl<E: 'static> Kind for IoK<E> {
    type F<A> = Io<E, A>;
}
