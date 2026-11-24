# task-1990 — Atomic record high · reverse

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Start an AtomicU32 at zero. For each byte perform fetch_max with the byte widened to u32. Observe the value returned by the operation (the value preceding this update). Arithmetic wraps in u32. Explain why relaxed ordering suffices for this single-cell trace.

Return the observations in reverse order. Retain duplicate observations. Use AtomicU32 read-modify-write operations and Ordering::Relaxed. During review, point to the operation that enforces its contract.

```rust
pub fn atomic_record_high_reverse(input: &[u8]) -> Vec<u32>
```

## Public examples

- `atomic_record_high_reverse(&[])` gives `vec![]`.
- `atomic_record_high_reverse(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1990/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1990`.

[Module reading](https://doc.rust-lang.org/std/sync/atomic/)
