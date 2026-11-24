# task-1516 — Packet low nibble · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

The package named track-wire is available under the local dependency alias wire. Use its public decode function. Observe the lower four bits from each byte. During review locate the package, import and filesystem names in the manifest.

Count adjacent pairs for which the second observation is strictly smaller. Use the bundled path dependency without duplicating its decoder. During review, point to the operation that enforces its contract.

```rust
pub fn packet_low_nibble_descending_steps(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `packet_low_nibble_descending_steps(&[])` gives `0`.
- `packet_low_nibble_descending_steps(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1516/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1516`.

[Module reading](https://doc.rust-lang.org/cargo/reference/workspaces.html)
