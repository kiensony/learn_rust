# task-1503 — Packet high nibble · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

The package named track-wire is available under the local dependency alias wire. Use its public decode function. Observe the upper four bits from each byte. During review locate the package, import and filesystem names in the manifest.

Return Some((minimum, maximum)) or None for an empty trace. Use the bundled path dependency without duplicating its decoder. During review, point to the operation that enforces its contract.

```rust
pub fn packet_high_nibble_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `packet_high_nibble_bounds(&[])` gives `None`.
- `packet_high_nibble_bounds(&[4])` gives `Some((0u32, 0u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1503/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1503`.

[Module reading](https://doc.rust-lang.org/cargo/reference/workspaces.html)
