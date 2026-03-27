//! Functor and Monad instances for the effect types.
//!
//! These impl blocks connect the practical `Io` type to the
//! categorical trait hierarchy in `foundation/`.

use crate::foundation::{Functor, Monad};
use super::io::{Io, IoK};

impl<E: 'static> Functor for IoK<E> {
    fn map<A: 'static, B: 'static>(fa: Io<E, A>, f: impl FnOnce(A) -> B + 'static) -> Io<E, B> {
        fa.map(f)
    }
}

impl<E: 'static> Monad for IoK<E> {
    fn pure<A: 'static>(a: A) -> Io<E, A> {
        Io::pure(a)
    }

    fn flat_map<A: 'static, B: 'static>(fa: Io<E, A>, f: impl FnOnce(A) -> Io<E, B> + 'static) -> Io<E, B> {
        fa.flat_map(f)
    }
}
