# task-2200 — Duplicate insertion cost · replay mismatch

Instrument an algorithm on a stream of keys. Observe the count of distinct keys stored after each insertion. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use deterministic operation counts independent of wall-clock time. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn duplicate_insertion_cost_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `duplicate_insertion_cost_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[1, 2, 3, 4, 5, 6, 7, 8, 9])` gives `Some(9)`.
- `duplicate_insertion_cost_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2200/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2200`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
