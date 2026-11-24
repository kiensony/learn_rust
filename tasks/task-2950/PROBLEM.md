# task-2950 — Rollback triggers · replay mismatch

An upgrade incident log records old and new latency ticks as consecutive bytes for each probe. Ignore a final incomplete pair. Observe one if new latency exceeds double the old, else zero per complete pair. Provide the smallest probe that triggered the starter fault and a regression test that protects it.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use paired measurements, widening or saturation appropriate to the metric. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn rollback_triggers_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `rollback_triggers_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[0, 0, 0, 1])` gives `Some(4)`.
- `rollback_triggers_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2950/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2950`.

[Module reading](https://doc.rust-lang.org/cargo/reference/resolver.html)
