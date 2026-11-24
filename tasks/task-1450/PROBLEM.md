# task-1450 — Deadline slack · replay mismatch

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the absolute difference in milliseconds from a 250-millisecond deadline.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use std::time::Duration without reading the wall clock. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn deadline_slack_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `deadline_slack_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[199, 148, 97, 46, 107, 5, 250, 192, 114])` gives `Some(9)`.
- `deadline_slack_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1450/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1450`.

[Module reading](https://doc.rust-lang.org/std/time/)
