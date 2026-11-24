# task-1510 — Packet high nibble · reverse

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

The package named track-wire is available under the local dependency alias wire. Use its public decode function. Observe the upper four bits from each byte. During review locate the package, import and filesystem names in the manifest.

Return the observations in reverse order. Retain duplicate observations. Use the bundled path dependency without duplicating its decoder. During review, point to the operation that enforces its contract.

```rust
pub fn packet_high_nibble_reverse(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `packet_high_nibble_reverse(&[])` gives `vec![]`.
- `packet_high_nibble_reverse(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1510/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1510`.

[Module reading](https://doc.rust-lang.org/cargo/reference/workspaces.html)
