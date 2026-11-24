# task-1720 — Saturating macro · reverse

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Write a local macro_rules! macro accepting an expression and evaluating it exactly once per input byte. For each byte observe the byte plus four with u8 saturation. In review, demonstrate the single-evaluation requirement with an expression that increments a counter.

Return the observations in reverse order. Retain duplicate observations. Use a local declarative macro with an expr matcher. During review, point to the operation that enforces its contract.

```rust
pub fn saturating_macro_reverse(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `saturating_macro_reverse(&[])` gives `vec![]`.
- `saturating_macro_reverse(&[4])` gives `vec![8u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1720/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1720`.

[Module reading](https://doc.rust-lang.org/reference/macros-by-example.html)
