# task-2297 — Hot code addresses · run ledger

Each input byte is a bucket ID recorded by a sampling profiler. Observe bucket IDs sampled at least twice, sorted by ID. During review explain the difference between sample counts and exact call counts. Benchmark runs should exclude formatting and file parsing.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use a histogram built from a deterministic profiler trace. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn hot_code_addresses_run_ledger(input: &[u8]) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `hot_code_addresses_run_ledger(&[9, 2, 2, 8, 0, 5])` gives `vec![(2u32, 1usize)]`.
- `hot_code_addresses_run_ledger(&[])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2297/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2297`.

[Module reading](https://doc.rust-lang.org/rustc/instrument-coverage.html)
