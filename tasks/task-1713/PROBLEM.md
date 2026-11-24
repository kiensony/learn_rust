# task-1713 — Saturating macro · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Write a local macro_rules! macro accepting an expression and evaluating it exactly once per input byte. For each byte observe the byte plus four with u8 saturation. In review, demonstrate the single-evaluation requirement with an expression that increments a counter.

Return Some((minimum, maximum)) or None for an empty trace. Use a local declarative macro with an expr matcher. During review, point to the operation that enforces its contract.

```rust
pub fn saturating_macro_bounds(input: &[u8]) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `saturating_macro_bounds(&[])` gives `None`.
- `saturating_macro_bounds(&[4])` gives `Some((8u32, 8u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1713/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1713`.

[Module reading](https://doc.rust-lang.org/reference/macros-by-example.html)
