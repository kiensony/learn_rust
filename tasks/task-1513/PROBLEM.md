# task-1513 — Packet low nibble · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

The package named track-wire is available under the local dependency alias wire. Use its public decode function. Observe the lower four bits from each byte. During review locate the package, import and filesystem names in the manifest.

Return Some((minimum, maximum)) or None for an empty trace. Use the bundled path dependency without duplicating its decoder. During review, point to the operation that enforces its contract.

```rust
pub fn packet_low_nibble_bounds(input: &[u8]) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `packet_low_nibble_bounds(&[])` gives `None`.
- `packet_low_nibble_bounds(&[4])` gives `Some((4u32, 4u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1513/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1513`.

[Module reading](https://doc.rust-lang.org/cargo/reference/workspaces.html)
