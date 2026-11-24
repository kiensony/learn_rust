# task-2150 — First occurrences · replay mismatch

A scheduler receives one u8 key per entry. Observe the sequence in which previously unseen keys first arrive. Explain the cost of inserting and extracting the next item.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use the standard library's HashSet membership and input-order iteration. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn first_occurrences_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `first_occurrences_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[3, 6, 9, 12, 21, 15, 0, 26, 8])` gives `Some(9)`.
- `first_occurrences_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2150/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2150`.

[Module reading](https://doc.rust-lang.org/std/collections/)
