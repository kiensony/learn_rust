# task-1992 — Atomic exchange · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Start an AtomicU32 at zero. For each byte perform swap with the byte widened to u32. Observe the value returned by the operation (the value preceding this update). Arithmetic wraps in u32. Explain why relaxed ordering suffices for this single-cell trace.

Return the sum of all observations, widening each one before adding. Use AtomicU32 read-modify-write operations and Ordering::Relaxed. During review, point to the operation that enforces its contract.

```rust
pub fn atomic_exchange_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `atomic_exchange_total(&[])` gives `0`.
- `atomic_exchange_total(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1992/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1992`.

[Module reading](https://doc.rust-lang.org/std/sync/atomic/)
