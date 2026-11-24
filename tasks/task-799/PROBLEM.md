# task-799 — Assemble a frequency leaderboard

Return the ID with the highest vote count. Break ties in favor of the smaller ID. Empty input yields None.

```rust
pub fn leader(votes: &[u8]) -> Option<u8>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `leader(&[4, 2, 4])` gives `Some(4)`.
- `leader(&[3, 2, 3, 2])` gives `Some(2)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-799/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-799`.

[Module reading](https://doc.rust-lang.org/std/collections/)
