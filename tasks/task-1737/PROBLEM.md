# task-1737 — Population macro · distinct

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Write a local macro_rules! macro accepting an expression and evaluating it exactly once per input byte. For each byte observe the number of one bits. In review, demonstrate the single-evaluation requirement with an expression that increments a counter.

Return distinct observations in ascending order. Use a local declarative macro with an expr matcher. During review, point to the operation that enforces its contract.

```rust
pub fn population_macro_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `population_macro_distinct(&[])` gives `vec![]`.
- `population_macro_distinct(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1737/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1737`.

[Module reading](https://doc.rust-lang.org/reference/macros-by-example.html)
