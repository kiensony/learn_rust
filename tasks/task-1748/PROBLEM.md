# task-1748 — Boolean macro · rolling load

Write a local macro_rules! macro accepting an expression and evaluating it exactly once per input byte. For each byte observe one for zero and zero otherwise. In review, demonstrate the single-evaluation requirement with an expression that increments a counter.

For each complete sliding window of width observations, return its sum as u64. Consecutive windows overlap by width minus one observations. Width zero and widths greater than the trace length yield an empty vector.

Use a local declarative macro with an expr matcher. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn boolean_macro_rolling_load(input: &[u8], width: usize) -> Vec<u64>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `boolean_macro_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1)` gives `vec![0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 1u64, 0u64, 0u64, 0u64]`.
- `boolean_macro_rolling_load(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1748/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1748`.

[Module reading](https://doc.rust-lang.org/reference/macros-by-example.html)
