# task-1800 — Token offset · replay mismatch

The provided arity::arity! procedural macro expands its input into the number of top-level token trees. Invoke it on `unit unit unit unit unit` to obtain the units. Observe every input byte increased by the generated value (currently 5). Do not hard-code the generated value.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use the supplied procedural macro at compile time. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn token_offset_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `token_offset_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[8, 11, 14, 17, 26, 20, 5, 31, 13])` gives `Some(9)`.
- `token_offset_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1800/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1800`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
