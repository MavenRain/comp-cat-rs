//! Stream: effectful iteration.
//!
//! `Stream<E, A>` produces zero or more values of type `A`,
//! where each step is an `Io<E, _>`.
//!
//! Categorically, a stream is a colimit (iterative construction),
//! hence a left Kan extension by `collapse::monad_is_kan`.
//!
//! The representation is pull-based: each stream is a suspended
//! `Io` that yields either `None` (done) or `Some((value, rest))`.

use std::rc::Rc;

use super::io::Io;
use crate::foundation::Kind;

/// Witness type for the Stream kind.
pub struct StreamK<E> {
    _phantom: core::marker::PhantomData<E>,
}

impl<E: 'static> Kind for StreamK<E> {
    type F<A> = Stream<E, A>;
}

/// The result of pulling one element from a stream.
type Step<E, A> = Io<E, Option<(A, Stream<E, A>)>>;

/// The step function type for `Stream::unfold`.
type UnfoldFn<E, A, S> = Rc<dyn Fn(S) -> Io<E, Option<(A, S)>>>;

/// An effectful stream producing values of type `A`.
///
/// Pull-based: each step is an `Io` that produces either
/// the next value and the rest of the stream, or `None`.
pub struct Stream<E, A> {
    step: Box<dyn FnOnce() -> Step<E, A>>,
}

impl<E: 'static, A: 'static> Stream<E, A> {
    /// An empty stream.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            step: Box::new(|| Io::pure(None)),
        }
    }

    /// A stream that emits a single value.
    #[must_use]
    pub fn emit(a: A) -> Self {
        Self {
            step: Box::new(move || Io::pure(Some((a, Self::empty())))),
        }
    }

    /// Build a stream from a `Vec` of values.
    #[must_use]
    pub fn from_vec(items: Vec<A>) -> Self {
        items
            .into_iter()
            .rev()
            .fold(Self::empty(), |rest, item| Self {
                step: Box::new(move || Io::pure(Some((item, rest)))),
            })
    }

    /// Build a stream by repeatedly applying a step function to state.
    ///
    /// The step function returns `None` to end the stream, or
    /// `Some((value, next_state))` to emit and continue.
    #[must_use]
    pub fn unfold<S: 'static>(
        init: S,
        step: UnfoldFn<E, A, S>,
    ) -> Self {
        let step_clone = Rc::clone(&step);
        Self {
            step: Box::new(move || {
                step(init).map(move |opt| {
                    opt.map(|(a, next_state)| (a, Self::unfold(next_state, step_clone)))
                })
            }),
        }
    }

    /// Lift a single `Io` into a one-element stream.
    #[must_use]
    pub fn from_io(io: Io<E, A>) -> Self {
        Self {
            step: Box::new(move || io.map(|a| Some((a, Self::empty())))),
        }
    }

    /// Apply a function to each element.
    #[must_use]
    pub fn map<B: 'static>(self, f: Rc<dyn Fn(A) -> B>) -> Stream<E, B> {
        let f_clone = Rc::clone(&f);
        Stream {
            step: Box::new(move || {
                self.pull().map(move |opt| {
                    opt.map(|(a, rest)| (f(a), rest.map(f_clone)))
                })
            }),
        }
    }

    /// Append another stream after this one.
    #[must_use]
    pub fn concat(self, other: Stream<E, A>) -> Self {
        Self {
            step: Box::new(move || {
                self.pull().flat_map(move |opt| match opt {
                    Some((a, rest)) => Io::pure(Some((a, rest.concat(other)))),
                    None => other.pull(),
                })
            }),
        }
    }

    /// Take at most `n` elements.
    #[must_use]
    pub fn take(self, n: usize) -> Self {
        match n {
            0 => Self::empty(),
            _ => Self {
                step: Box::new(move || {
                    self.pull().map(move |opt| {
                        opt.map(|(a, rest)| (a, rest.take(n - 1)))
                    })
                }),
            },
        }
    }

    /// Collapse the stream into a single value via a folding function.
    ///
    /// This is the primary way to "run" a stream, producing an `Io`.
    #[must_use]
    pub fn fold<B: 'static>(self, init: B, f: Rc<dyn Fn(B, A) -> B>) -> Io<E, B> {
        self.pull().flat_map(move |opt| match opt {
            None => Io::pure(init),
            Some((a, rest)) => {
                let next = f(init, a);
                rest.fold(next, f)
            }
        })
    }

    /// Collect all stream elements into a `Vec`.
    #[must_use]
    pub fn collect(self) -> Io<E, Vec<A>> {
        self.fold(Vec::new(), Rc::new(|acc, a| {
            acc.into_iter().chain(std::iter::once(a)).collect()
        }))
    }

    /// Pull the next element from the stream.
    ///
    /// Returns the `Io` that produces the next step.
    fn pull(self) -> Step<E, A> {
        (self.step)()
    }
}
