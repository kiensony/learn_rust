# task-2197 — Sorted insertion moves · run ledger

Instrument an algorithm on a stream of keys. Observe the number of older elements shifted right when each key is inserted stably after any equals in a sorted vector. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use deterministic operation counts independent of wall-clock time. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn sorted_insertion_moves_run_ledger(input: &[u8]) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `sorted_insertion_moves_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31])` gives `vec![(0u32, 5usize), (1u32, 1usize), (6u32, 1usize), (0u32, 1usize), (5u32, 1usize), (0u32, 1usize)]`.
- `sorted_insertion_moves_run_ledger(&[])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2197/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2197`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
