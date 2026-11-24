# task-1528 — Packet parity · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

The package named track-wire is available under the local dependency alias wire. Use its public decode function. Observe one for odd bit parity and zero for even bit parity from each byte. During review locate the package, import and filesystem names in the manifest.

Return the absolute differences between each adjacent pair, in order. Use the bundled path dependency without duplicating its decoder. During review, point to the operation that enforces its contract.

```rust
pub fn packet_parity_steps(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `packet_parity_steps(&[])` gives `vec![]`.
- `packet_parity_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1528/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1528`.

[Module reading](https://doc.rust-lang.org/cargo/reference/workspaces.html)
