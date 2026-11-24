# task-1529 — Packet parity · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

The package named track-wire is available under the local dependency alias wire. Use its public decode function. Observe one for odd bit parity and zero for even bit parity from each byte. During review locate the package, import and filesystem names in the manifest.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use the bundled path dependency without duplicating its decoder. During review, point to the operation that enforces its contract.

```rust
pub fn packet_parity_longest_run(input: &[u8]) -> u64
```

## Public examples

- `packet_parity_longest_run(&[])` gives `0`.
- `packet_parity_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1529/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1529`.

[Module reading](https://doc.rust-lang.org/cargo/reference/workspaces.html)
