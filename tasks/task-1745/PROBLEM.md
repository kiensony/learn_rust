# task-1745 — Rotate macro · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Write a local macro_rules! macro accepting an expression and evaluating it exactly once per input byte. For each byte observe the byte rotated left two bits. In review, demonstrate the single-evaluation requirement with an expression that increments a counter.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use a local declarative macro with an expr matcher. During review, point to the operation that enforces its contract.

```rust
pub fn rotate_macro_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `rotate_macro_changes(&[])` gives `vec![]`.
- `rotate_macro_changes(&[4])` gives `vec![16u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1745/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1745`.

[Module reading](https://doc.rust-lang.org/reference/macros-by-example.html)
