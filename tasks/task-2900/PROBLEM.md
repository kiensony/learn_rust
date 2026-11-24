# task-2900 — Signed sensor magnitude · replay mismatch

Decode a portable register image for a host and WebAssembly consumer. Each record occupies two bytes. Ignore a trailing unpaired byte. Observe the absolute magnitude of the little-endian signed 16-bit reading per complete record. Verify with cargo check --target wasm32-unknown-unknown after installing the target, and explain the byte order contract.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use fixed-width integer operations independent of host byte order. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn signed_sensor_magnitude_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `signed_sensor_magnitude_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[1539, 3081, 3861, 6656])` gives `Some(4)`.
- `signed_sensor_magnitude_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2900/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2900`.

[Module reading](https://doc.rust-lang.org/rustc/platform-support.html)
