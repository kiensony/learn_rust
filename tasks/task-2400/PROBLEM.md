# task-2400 — Last change age · replay mismatch

Collect a cumulative trace over a byte stream. At each position observe the length of the current run of consecutive equal bytes. Use a single pass with constant extra bookkeeping beyond the result. Add a slow prefix-by-prefix oracle to a test and compare on a stream of at least 100 entries. Explain the number of element visits in each implementation.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use incremental state and an independent performance model. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn last_change_age_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `last_change_age_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[1, 1, 1, 1, 1, 1, 1, 1, 1])` gives `Some(9)`.
- `last_change_age_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2400/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2400`.

[Module reading](https://doc.rust-lang.org/std/time/struct.Instant.html)
