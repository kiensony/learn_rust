# task-2397 — Single pass prefix minimum · run ledger

Collect a cumulative trace over a byte stream. At each position observe the minimum value seen so far. Use a single pass with constant extra bookkeeping beyond the result. Add a slow prefix-by-prefix oracle to a test and compare on a stream of at least 100 entries. Explain the number of element visits in each implementation.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use incremental state and an independent performance model. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn single_pass_prefix_minimum_run_ledger(input: &[u8]) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `single_pass_prefix_minimum_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31])` gives `vec![(3u32, 6usize), (0u32, 4usize)]`.
- `single_pass_prefix_minimum_run_ledger(&[])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2397/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2397`.

[Module reading](https://doc.rust-lang.org/std/time/struct.Instant.html)
