# task-1712 — Saturating macro · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Write a local macro_rules! macro accepting an expression and evaluating it exactly once per input byte. For each byte observe the byte plus four with u8 saturation. In review, demonstrate the single-evaluation requirement with an expression that increments a counter.

Return the sum of all observations, widening each one before adding. Use a local declarative macro with an expr matcher. During review, point to the operation that enforces its contract.

```rust
pub fn saturating_macro_total(input: &[u8]) -> u64
```

## Public examples

- `saturating_macro_total(&[])` gives `0`.
- `saturating_macro_total(&[4])` gives `8`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1712/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1712`.

[Module reading](https://doc.rust-lang.org/reference/macros-by-example.html)
