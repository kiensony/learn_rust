# task-1999 — Atomic record high · frequent readings

Start an AtomicU32 at zero. For each byte perform fetch_max with the byte widened to u32. Observe the value returned by the operation (the value preceding this update). Arithmetic wraps in u32. Explain why relaxed ordering suffices for this single-cell trace.

Return up to keep distinct observations ordered by descending frequency, breaking ties by first appearance in the trace. Return an empty vector for keep zero. Each observation value appears at most once in the result.

Use AtomicU32 read-modify-write operations and Ordering::Relaxed. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn atomic_record_high_frequent_readings(input: &[u8], keep: usize) -> Vec<u32>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `atomic_record_high_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3)` gives `vec![21u32, 26u32, 0u32]`.
- `atomic_record_high_frequent_readings(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1999/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1999`.

[Module reading](https://doc.rust-lang.org/std/sync/atomic/)
