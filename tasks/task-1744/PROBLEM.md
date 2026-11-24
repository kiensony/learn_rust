# task-1744 — Rotate macro · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Write a local macro_rules! macro accepting an expression and evaluating it exactly once per input byte. For each byte observe the byte rotated left two bits. In review, demonstrate the single-evaluation requirement with an expression that increments a counter.

Keep the first observation and every later one strictly above all earlier observations. Use a local declarative macro with an expr matcher. During review, point to the operation that enforces its contract.

```rust
pub fn rotate_macro_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `rotate_macro_peaks(&[])` gives `vec![]`.
- `rotate_macro_peaks(&[4])` gives `vec![16u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1744/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1744`.

[Module reading](https://doc.rust-lang.org/reference/macros-by-example.html)
