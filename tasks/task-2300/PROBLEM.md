# task-2300 — Profile ranking · replay mismatch

Each input byte is a bucket ID recorded by a sampling profiler. Observe bucket IDs in descending sample-count order, breaking ties with the larger ID first. During review explain the difference between sample counts and exact call counts. Benchmark runs should exclude formatting and file parsing.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use a histogram built from a deterministic profiler trace. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn profile_ranking_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `profile_ranking_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[31, 26, 21, 15, 12, 9, 8, 6, 3])` gives `Some(9)`.
- `profile_ranking_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2300/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2300`.

[Module reading](https://doc.rust-lang.org/rustc/instrument-coverage.html)
