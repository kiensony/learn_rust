# task-1650 — Shift property · replay mismatch

Each byte is a generated test input. Observe one when the following predicate holds and zero otherwise: shifting right then left by one changes the value. As a separate regression test, enumerate the full u8 domain; check the property against an independently computed model and report the smallest counterexample.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use an exhaustive u8 domain test with a model separate from the implementation. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn shift_property_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `shift_property_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[1, 0, 1, 0, 1, 1, 0, 0, 0])` gives `Some(9)`.
- `shift_property_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1650/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1650`.

[Module reading](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
