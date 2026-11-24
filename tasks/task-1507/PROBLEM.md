# task-1507 — Packet high nibble · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

The package named track-wire is available under the local dependency alias wire. Use its public decode function. Observe the upper four bits from each byte. During review locate the package, import and filesystem names in the manifest.

Return distinct observations in ascending order. Use the bundled path dependency without duplicating its decoder. During review, point to the operation that enforces its contract.

```rust
pub fn packet_high_nibble_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `packet_high_nibble_distinct(&[])` gives `vec![]`.
- `packet_high_nibble_distinct(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1507/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1507`.

[Module reading](https://doc.rust-lang.org/cargo/reference/workspaces.html)
