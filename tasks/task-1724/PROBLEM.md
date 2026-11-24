# task-1724 — Boolean macro · peaks

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Write a local macro_rules! macro accepting an expression and evaluating it exactly once per input byte. For each byte observe one for zero and zero otherwise. In review, demonstrate the single-evaluation requirement with an expression that increments a counter.

Keep the first observation and every later one strictly above all earlier observations. Use a local declarative macro with an expr matcher. During review, point to the operation that enforces its contract.

```rust
pub fn boolean_macro_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `boolean_macro_peaks(&[])` gives `vec![]`.
- `boolean_macro_peaks(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1724/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1724`.

[Module reading](https://doc.rust-lang.org/reference/macros-by-example.html)
