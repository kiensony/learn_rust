# task-1718 — Saturating macro · steps

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Write a local macro_rules! macro accepting an expression and evaluating it exactly once per input byte. For each byte observe the byte plus four with u8 saturation. In review, demonstrate the single-evaluation requirement with an expression that increments a counter.

Return the absolute differences between each adjacent pair, in order. Use a local declarative macro with an expr matcher. During review, point to the operation that enforces its contract.

```rust
pub fn saturating_macro_steps(input: &[u8]) -> Vec<u32>
```

## Public examples

- `saturating_macro_steps(&[])` gives `vec![]`.
- `saturating_macro_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1718/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1718`.

[Module reading](https://doc.rust-lang.org/reference/macros-by-example.html)
