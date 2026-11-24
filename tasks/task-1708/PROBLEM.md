# task-1708 — Nibble macro · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Write a local macro_rules! macro accepting an expression and evaluating it exactly once per input byte. For each byte observe the low nibble. In review, demonstrate the single-evaluation requirement with an expression that increments a counter.

Return the absolute differences between each adjacent pair, in order. Use a local declarative macro with an expr matcher. During review, point to the operation that enforces its contract.

```rust
pub fn nibble_macro_steps(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `nibble_macro_steps(&[])` gives `vec![]`.
- `nibble_macro_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1708/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1708`.

[Module reading](https://doc.rust-lang.org/reference/macros-by-example.html)
