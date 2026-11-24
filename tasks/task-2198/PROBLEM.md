# task-2198 — Prefix comparison budget · rolling load

Instrument an algorithm on a stream of keys. Observe the exact count of comparisons made when each new entry is compared to every earlier entry. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

For each complete sliding window of width observations, return its sum as u64. Consecutive windows overlap by width minus one observations. Width zero and widths greater than the trace length yield an empty vector.

Use deterministic operation counts independent of wall-clock time. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn prefix_comparison_budget_rolling_load(input: &[u8], width: usize) -> Vec<u64>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `prefix_comparison_budget_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1)` gives `vec![0u64, 1u64, 2u64, 3u64, 4u64, 5u64, 6u64, 7u64, 8u64, 9u64]`.
- `prefix_comparison_budget_rolling_load(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2198/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2198`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
