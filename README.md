# comp-cat-rs

Computational category theory in Rust: a Cats-Effect/ZIO-style effect system whose design is justified by the thesis that every construction in category theory is a Kan extension.

The categorical foundations are formalized and proved in [comp-cat-theory](https://github.com/MavenRain/comp-cat-theory) (Lean 4, zero `sorry`s).  This crate is the Rust implementation: practical types for effectful programming, grounded in that formalization.

## Installation

```toml
[dependencies]
comp-cat-rs = "0.1"
```

## Quick start

```rust
use comp_cat_rs::effect::io::Io;

// Pure values
let greeting: Io<String, &str> = Io::pure("hello");

// Suspended effects
let read_env: Io<String, String> = Io::suspend(|| {
    std::env::var("HOME").map_err(|e| e.to_string())
});

// Composition via flat_map
let program: Io<String, String> = read_env.map(|home| {
    format!("Home directory: {home}")
});

// Nothing runs until you call .run()
let result: Result<String, String> = program.run();
```

## Architecture

```
foundation/     Kind, Functor, Monad traits (HKT via GATs)
primitive/      Kan extension types (theoretical layer)
collapse/       Design justification: every concept is a Kan extension
effect/         The practical payoff: Io, Resource, Stream, Fiber
```

The `effect/` layer is why this crate exists.  The other three layers explain *why* the effect types are the right abstractions.

## Core types

### `Io<E, A>` -- effectful computation

A lazy, composable computation that produces `A` or fails with `E`.  Nothing executes until `.run()` is called.

```rust
use comp_cat_rs::effect::io::Io;

#[derive(Debug)]
enum AppError {
    NotFound(String),
    ParseFailed(std::num::ParseIntError),
}

// Suspend a side effect
let read_file: Io<AppError, String> = Io::suspend(|| {
    std::fs::read_to_string("config.txt")
        .map_err(|_| AppError::NotFound("config.txt".into()))
});

// Chain computations
let parsed: Io<AppError, u64> = read_file.flat_map(|contents| {
    Io::suspend(move || {
        contents.trim().parse::<u64>().map_err(AppError::ParseFailed)
    })
});

// Error handling
let with_default: Io<AppError, u64> = parsed.handle_error(|_| 42);

// Combine two computations
let zipped: Io<AppError, (u64, u64)> = Io::pure(10).zip(Io::pure(20));
```

**Combinators:**

| Method | Signature | Description |
|--------|-----------|-------------|
| `pure` | `A -> Io<E, A>` | Lift a value |
| `suspend` | `(() -> Result<A, E>) -> Io<E, A>` | Suspend a side effect |
| `map` | `Io<E, A> -> (A -> B) -> Io<E, B>` | Transform the value |
| `flat_map` | `Io<E, A> -> (A -> Io<E, B>) -> Io<E, B>` | Monadic bind |
| `zip` | `Io<E, A> -> Io<E, B> -> Io<E, (A, B)>` | Sequential pair |
| `attempt` | `Io<E, A> -> Io<Infallible, Result<A, E>>` | Capture errors |
| `handle_error` | `Io<E, A> -> (E -> A) -> Io<E, A>` | Pure recovery |
| `handle_error_with` | `Io<E, A> -> (E -> Io<E2, A>) -> Io<E2, A>` | Effectful recovery |
| `map_error` | `Io<E, A> -> (E -> E2) -> Io<E2, A>` | Transform error type |
| `as_unit` | `Io<E, A> -> Io<E, ()>` | Discard value |
| `run` | `Io<E, A> -> Result<A, E>` | Execute (side effects happen here) |

### `Resource<E, A>` -- bracket-based resource management

Guarantees release after use, even on error.

```rust
use comp_cat_rs::effect::io::Io;
use comp_cat_rs::effect::resource::Resource;

let file = Resource::make(
    || Io::suspend(|| {  // acquire
        std::fs::File::open("data.txt").map_err(|e| e.to_string())
    }),
    |_handle| Io::pure(()),  // release
);

let contents: Io<String, String> = file.use_resource(|_handle| {
    Io::pure("file contents here".to_string())
});
```

### `Stream<E, A>` -- effectful iteration

A pull-based stream where each step is an `Io`.

```rust
use std::rc::Rc;
use comp_cat_rs::effect::stream::Stream;
use comp_cat_rs::effect::io::Io;

// From a vec
let s: Stream<String, i32> = Stream::from_vec(vec![1, 2, 3, 4, 5]);

// Take, fold, collect
let sum: Io<String, i32> = s.take(3).fold(0, Rc::new(|acc, x| acc + x));
let result = sum.run();  // Ok(6)

// Unfold from state
let countdown: Stream<String, i32> = Stream::unfold(
    5,
    Rc::new(|n| Io::pure(
        if n > 0 { Some((n, n - 1)) } else { None }
    )),
);
let collected = countdown.collect().run();  // Ok(vec![5, 4, 3, 2, 1])
```

**Constructors and combinators:**

| Function | Description |
|----------|-------------|
| `Stream::empty()` | Empty stream |
| `Stream::emit(a)` | Single-element stream |
| `Stream::from_vec(v)` | Stream from a vector |
| `Stream::from_io(io)` | Single-element stream from an `Io` |
| `Stream::unfold(init, step)` | Build from state + step function |
| `.map(f)` | Transform each element |
| `.concat(other)` | Append another stream |
| `.take(n)` | First n elements |
| `.fold(init, f)` | Collapse to `Io<E, B>` |
| `.collect()` | Collect to `Io<E, Vec<A>>` |

### `Fiber<E, A>` -- lightweight concurrency

Fork an `Io` onto a thread, join later.

```rust
use comp_cat_rs::effect::io::Io;
use comp_cat_rs::effect::fiber::{Fiber, par_zip};

let task_a: Io<String, i32> = Io::pure(1);
let task_b: Io<String, i32> = Io::pure(2);

// Run two computations concurrently
let both = par_zip(task_a, task_b);  // Io<FiberError<String>, (i32, i32)>
```

**`FiberError<E>`** covers three failure modes:
- `Failed(E)` -- the computation itself failed
- `Panicked(String)` -- the thread panicked
- `SpawnFailed(std::io::Error)` -- OS refused to create a thread

## Trait hierarchy

```
Kind              type constructor: A |-> F<A>
  |
Functor           map: F<A> -> (A -> B) -> F<B>
  |
Monad             pure: A -> F<A>
                  flat_map: F<A> -> (A -> F<B>) -> F<B>
```

`IoK<E>` and `StreamK<E>` implement `Monad`, so you can write code generic over any monad.

## The categorical thesis

Every type in this crate is a specific Kan extension:

```
Monad       = Eilenberg-Moore adjunction     = (Lan, Ran) pair
Io          = free monad                     = left adjoint   = Ran
Resource    = bracket adjunction             = (Lan, Ran) pair
Stream      = colimit (iterative unfolding)  = Lan
Fiber::fork = coproduct                      = Lan
Fiber::join = limit                          = Ran
```

The proofs live in [comp-cat-theory](https://github.com/MavenRain/comp-cat-theory), a Lean 4 formalization with zero `sorry`s and 18 files that collapse all of computational category theory to a single primitive: `KanExtension.lean`.

## License

MIT
