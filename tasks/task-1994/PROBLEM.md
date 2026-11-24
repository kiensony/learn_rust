# task-1994 — Atomic exchange · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Start an AtomicU32 at zero. For each byte perform swap with the byte widened to u32. Observe the value returned by the operation (the value preceding this update). Arithmetic wraps in u32. Explain why relaxed ordering suffices for this single-cell trace.

Keep the first observation and every later one strictly above all earlier observations. Use AtomicU32 read-modify-write operations and Ordering::Relaxed. During review, point to the operation that enforces its contract.

```rust
pub fn atomic_exchange_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `atomic_exchange_peaks(&[])` gives `vec![]`.
- `atomic_exchange_peaks(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1994/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1994`.

[Module reading](https://doc.rust-lang.org/std/sync/atomic/)
