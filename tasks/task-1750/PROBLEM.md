# task-1750 — Rotate macro · replay mismatch

Write a local macro_rules! macro accepting an expression and evaluating it exactly once per input byte. For each byte observe the byte rotated left two bits. In review, demonstrate the single-evaluation requirement with an expression that increments a counter.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use a local declarative macro with an expr matcher. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn rotate_macro_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `rotate_macro_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[12, 24, 36, 48, 84, 60, 0, 104, 32])` gives `Some(9)`.
- `rotate_macro_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1750/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1750`.

[Module reading](https://doc.rust-lang.org/reference/macros-by-example.html)
