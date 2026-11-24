# task-1749 — Population macro · frequent readings

Write a local macro_rules! macro accepting an expression and evaluating it exactly once per input byte. For each byte observe the number of one bits. In review, demonstrate the single-evaluation requirement with an expression that increments a counter.

Return up to keep distinct observations ordered by descending frequency, breaking ties by first appearance in the trace. Return an empty vector for keep zero. Each observation value appears at most once in the result.

Use a local declarative macro with an expr matcher. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn population_macro_frequent_readings(input: &[u8], keep: usize) -> Vec<u32>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `population_macro_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3)` gives `vec![2u32, 3u32, 4u32]`.
- `population_macro_frequent_readings(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1749/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1749`.

[Module reading](https://doc.rust-lang.org/reference/macros-by-example.html)
