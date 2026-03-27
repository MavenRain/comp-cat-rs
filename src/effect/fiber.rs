//! Fiber: lightweight concurrency via fork/join.
//!
//! `Fiber<E, A>` is a handle to a computation running on another
//! thread.  It provides `join` to await the result.
//!
//! Categorically, forking is a coproduct (colimit) in the category
//! of computations, and joining is a limit.  Both are Kan extensions.

use super::io::Io;

/// Errors that can occur when working with fibers.
#[derive(Debug)]
pub enum FiberError<E> {
    /// The computation failed with its own error type.
    Failed(E),
    /// The thread panicked.
    Panicked(String),
    /// The OS refused to spawn a thread.
    SpawnFailed(std::io::Error),
}

impl<E: core::fmt::Display> core::fmt::Display for FiberError<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Failed(e) => write!(f, "fiber failed: {e}"),
            Self::Panicked(msg) => write!(f, "fiber panicked: {msg}"),
            Self::SpawnFailed(e) => write!(f, "fiber spawn failed: {e}"),
        }
    }
}

impl<E: core::fmt::Debug + core::fmt::Display> std::error::Error for FiberError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SpawnFailed(e) => Some(e),
            Self::Failed(_) | Self::Panicked(_) => None,
        }
    }
}

/// A handle to a computation running on another thread.
///
/// Created via `Fiber::fork`, consumed via `Fiber::join`.
pub struct Fiber<E, A> {
    handle: std::thread::JoinHandle<Result<A, E>>,
}

impl<E: Send + 'static, A: Send + 'static> Fiber<E, A> {
    /// Fork an `Io` computation onto a new thread.
    ///
    /// Returns an `Io` that, when run, spawns the thread and
    /// produces a `Fiber` handle.
    ///
    /// The `Send` bounds are required because the computation
    /// crosses a thread boundary.
    ///
    /// # Errors
    ///
    /// Returns `FiberError::SpawnFailed` if the OS cannot create
    /// a new thread.
    #[must_use]
    pub fn fork(io: Io<E, A>) -> Io<FiberError<E>, Self>
    where
        Io<E, A>: Send,
    {
        Io::suspend(move || {
            std::thread::Builder::new()
                .spawn(move || io.run())
                .map(|handle| Self { handle })
                .map_err(FiberError::SpawnFailed)
        })
    }

    /// Join the fiber, waiting for its result.
    ///
    /// Consumes the fiber handle.
    ///
    /// # Errors
    ///
    /// - `FiberError::Failed(e)` if the computation returned `Err(e)`
    /// - `FiberError::Panicked(msg)` if the thread panicked
    #[must_use]
    pub fn join(self) -> Io<FiberError<E>, A> {
        Io::suspend(move || {
            self.handle
                .join()
                .map_err(|panic| {
                    let msg = panic
                        .downcast_ref::<String>()
                        .map(String::as_str)
                        .or_else(|| panic.downcast_ref::<&str>().copied())
                        .unwrap_or("unknown panic")
                        .to_owned();
                    FiberError::Panicked(msg)
                })
                .and_then(|result| result.map_err(FiberError::Failed))
        })
    }
}

/// Fork two computations and combine their results.
///
/// Both run concurrently on separate threads.
///
/// # Errors
///
/// Returns a `FiberError` if either computation fails, panics,
/// or if a thread cannot be spawned.
pub fn par_zip<E, A, B>(
    io_a: Io<E, A>,
    io_b: Io<E, B>,
) -> Io<FiberError<E>, (A, B)>
where
    E: Send + 'static,
    A: Send + 'static,
    B: Send + 'static,
    Io<E, A>: Send,
    Io<E, B>: Send,
{
    Fiber::fork(io_a).flat_map(move |fiber_a| {
        Fiber::fork(io_b).flat_map(move |fiber_b| {
            fiber_a.join().flat_map(move |a| {
                fiber_b.join().map(move |b| (a, b))
            })
        })
    })
}
