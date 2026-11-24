# task-1743 — Rotate macro · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Write a local macro_rules! macro accepting an expression and evaluating it exactly once per input byte. For each byte observe the byte rotated left two bits. In review, demonstrate the single-evaluation requirement with an expression that increments a counter.

Return Some((minimum, maximum)) or None for an empty trace. Use a local declarative macro with an expr matcher. During review, point to the operation that enforces its contract.

```rust
pub fn rotate_macro_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `rotate_macro_bounds(&[])` gives `None`.
- `rotate_macro_bounds(&[4])` gives `Some((16u32, 16u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1743/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1743`.

[Module reading](https://doc.rust-lang.org/reference/macros-by-example.html)
