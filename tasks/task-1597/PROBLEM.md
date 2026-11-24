# task-1597 — Feature padding · run ledger

Each byte counts units. Observe the byte value multiplied by the active unit width. The default width is one. With the wide feature it is 4. Add a cfg-gated regression for wide and run cargo test --features wide from this package. Test both configurations. Put your own feature-specific case in a new integration test.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use Cargo features and cfg!. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn feature_padding_run_ledger(input: &[u8]) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `feature_padding_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31])` gives `vec![(3u32, 1usize), (6u32, 1usize), (9u32, 1usize), (12u32, 1usize), (21u32, 1usize), (15u32, 1usize), (0u32, 1usize), (26u32, 1usize), (8u32, 1usize), (31u32, 1usize)]` by default and `vec![(12u32, 1usize), (24u32, 1usize), (36u32, 1usize), (48u32, 1usize), (84u32, 1usize), (60u32, 1usize), (0u32, 1usize), (104u32, 1usize), (32u32, 1usize), (124u32, 1usize)]` with `wide`.
- `feature_padding_run_ledger(&[])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1597/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1597`.

[Module reading](https://doc.rust-lang.org/cargo/reference/features.html)
