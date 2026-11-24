# task-1525 — Packet parity · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

The package named track-wire is available under the local dependency alias wire. Use its public decode function. Observe one for odd bit parity and zero for even bit parity from each byte. During review locate the package, import and filesystem names in the manifest.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use the bundled path dependency without duplicating its decoder. During review, point to the operation that enforces its contract.

```rust
pub fn packet_parity_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `packet_parity_changes(&[])` gives `vec![]`.
- `packet_parity_changes(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1525/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1525`.

[Module reading](https://doc.rust-lang.org/cargo/reference/workspaces.html)
