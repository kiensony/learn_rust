# task-1747 — Saturating macro · run ledger

Write a local macro_rules! macro accepting an expression and evaluating it exactly once per input byte. For each byte observe the byte plus four with u8 saturation. In review, demonstrate the single-evaluation requirement with an expression that increments a counter.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use a local declarative macro with an expr matcher. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn saturating_macro_run_ledger(input: &[u8]) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `saturating_macro_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31])` gives `vec![(7u32, 1usize), (10u32, 1usize), (13u32, 1usize), (16u32, 1usize), (25u32, 1usize), (19u32, 1usize), (4u32, 1usize), (30u32, 1usize), (12u32, 1usize), (35u32, 1usize)]`.
- `saturating_macro_run_ledger(&[])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1747/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1747`.

[Module reading](https://doc.rust-lang.org/reference/macros-by-example.html)
