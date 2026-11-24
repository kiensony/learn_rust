# task-1517 — Packet low nibble · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

The package named track-wire is available under the local dependency alias wire. Use its public decode function. Observe the lower four bits from each byte. During review locate the package, import and filesystem names in the manifest.

Return distinct observations in ascending order. Use the bundled path dependency without duplicating its decoder. During review, point to the operation that enforces its contract.

```rust
pub fn packet_low_nibble_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `packet_low_nibble_distinct(&[])` gives `vec![]`.
- `packet_low_nibble_distinct(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1517/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1517`.

[Module reading](https://doc.rust-lang.org/cargo/reference/workspaces.html)
