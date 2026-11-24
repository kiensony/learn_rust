# task-1991 — Atomic exchange · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Start an AtomicU32 at zero. For each byte perform swap with the byte widened to u32. Observe the value returned by the operation (the value preceding this update). Arithmetic wraps in u32. Explain why relaxed ordering suffices for this single-cell trace.

Return the observations in input order, including duplicates. Use AtomicU32 read-modify-write operations and Ordering::Relaxed. During review, point to the operation that enforces its contract.

```rust
pub fn atomic_exchange_trace(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `atomic_exchange_trace(&[])` gives `vec![]`.
- `atomic_exchange_trace(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1991/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1991`.

[Module reading](https://doc.rust-lang.org/std/sync/atomic/)
