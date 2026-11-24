# task-2199 — Early exit probes · frequent readings

Instrument an algorithm on a stream of keys. Observe the number of earlier entries inspected left-to-right, stopping at the first that is strictly greater than the current key, or after all earlier entries. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Return up to keep distinct observations ordered by descending frequency, breaking ties by first appearance in the trace. Return an empty vector for keep zero. Each observation value appears at most once in the result.

Use deterministic operation counts independent of wall-clock time. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn early_exit_probes_frequent_readings(input: &[u8], keep: usize) -> Vec<u32>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `early_exit_probes_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3)` gives `vec![1u32, 3u32, 0u32]`.
- `early_exit_probes_frequent_readings(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2199/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2199`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
