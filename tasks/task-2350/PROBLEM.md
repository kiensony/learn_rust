# task-2350 — Baseline drift · replay mismatch

The input is a fixed sequence of benchmark durations in microseconds. Observe the absolute difference between each duration and the first measurement. Run the acceptance suite with both cargo test and cargo test --release, and explain how compiler optimization could affect a timing-based assertion.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use deterministic measurement fixtures and a release build. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn baseline_drift_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `baseline_drift_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[0, 3, 6, 9, 18, 12, 3, 23, 5])` gives `Some(9)`.
- `baseline_drift_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2350/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2350`.

[Module reading](https://doc.rust-lang.org/cargo/reference/profiles.html)
