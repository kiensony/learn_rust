# task-1722 — Boolean macro · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Write a local macro_rules! macro accepting an expression and evaluating it exactly once per input byte. For each byte observe one for zero and zero otherwise. In review, demonstrate the single-evaluation requirement with an expression that increments a counter.

Return the sum of all observations, widening each one before adding. Use a local declarative macro with an expr matcher. During review, point to the operation that enforces its contract.

```rust
pub fn boolean_macro_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `boolean_macro_total(&[])` gives `0`.
- `boolean_macro_total(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1722/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1722`.

[Module reading](https://doc.rust-lang.org/reference/macros-by-example.html)
