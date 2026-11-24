# task-1544 — Packet reversal · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

The package named track-wire is available under the local dependency alias wire. Use its public decode function. Observe the reversed eight-bit pattern from each byte. During review locate the package, import and filesystem names in the manifest.

Keep the first observation and every later one strictly above all earlier observations. Use the bundled path dependency without duplicating its decoder. During review, point to the operation that enforces its contract.

```rust
pub fn packet_reversal_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `packet_reversal_peaks(&[])` gives `vec![]`.
- `packet_reversal_peaks(&[4])` gives `vec![32u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1544/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1544`.

[Module reading](https://doc.rust-lang.org/cargo/reference/workspaces.html)
