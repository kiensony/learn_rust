# task-1250 — Limited reads · replay mismatch

A bounded preview may consume at most four bytes. Observe every consumed byte including zero.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use Read::take and Cursor. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn limited_reads_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `limited_reads_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[3, 6, 9])` gives `Some(3)`.
- `limited_reads_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1250/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1250`.

[Module reading](https://doc.rust-lang.org/std/io/)
