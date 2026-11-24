# task-1518 — Packet low nibble · steps

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

The package named track-wire is available under the local dependency alias wire. Use its public decode function. Observe the lower four bits from each byte. During review locate the package, import and filesystem names in the manifest.

Return the absolute differences between each adjacent pair, in order. Use the bundled path dependency without duplicating its decoder. During review, point to the operation that enforces its contract.

```rust
pub fn packet_low_nibble_steps(input: &[u8]) -> Vec<u32>
```

## Public examples

- `packet_low_nibble_steps(&[])` gives `vec![]`.
- `packet_low_nibble_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1518/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1518`.

[Module reading](https://doc.rust-lang.org/cargo/reference/workspaces.html)
