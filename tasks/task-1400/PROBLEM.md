# task-1400 — Frame maxima · replay mismatch

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its largest payload byte, or zero for an empty frame.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use slice range checks before consuming a frame. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn frame_maxima_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `frame_maxima_replay_mismatch(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2], &[7, 4, 0])` gives `Some(3)`.
- `frame_maxima_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1400/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1400`.

[Module reading](https://doc.rust-lang.org/std/net/)
