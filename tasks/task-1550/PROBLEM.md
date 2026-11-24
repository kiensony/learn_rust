# task-1550 — Packet reversal · replay mismatch

The package named track-wire is available under the local dependency alias wire. Use its public decode function. Observe the reversed eight-bit pattern from each byte. During review locate the package, import and filesystem names in the manifest.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use the bundled path dependency without duplicating its decoder. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn packet_reversal_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `packet_reversal_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[192, 96, 144, 48, 168, 240, 0, 88, 16])` gives `Some(9)`.
- `packet_reversal_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1550/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1550`.

[Module reading](https://doc.rust-lang.org/cargo/reference/workspaces.html)
