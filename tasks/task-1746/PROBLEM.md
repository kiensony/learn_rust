# task-1746 — Nibble macro · threshold crossing

Write a local macro_rules! macro accepting an expression and evaluating it exactly once per input byte. For each byte observe the low nibble. In review, demonstrate the single-evaluation requirement with an expression that increments a counter.

Return the zero-based position and value of the first observation at least limit. Return None if no observation qualifies. The position refers to the derived sequence.

Use a local declarative macro with an expr matcher. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn nibble_macro_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `nibble_macro_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0)` gives `Some((0, 3))`.
- `nibble_macro_threshold_crossing(&[], 0)` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1746/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1746`.

[Module reading](https://doc.rust-lang.org/reference/macros-by-example.html)
