# task-1379 — Frame checksum · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its payload XOR, starting at zero.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use slice range checks before consuming a frame. During review, point to the operation that enforces its contract.

```rust
pub fn frame_checksum_longest_run(input: &[u8]) -> u64
```

## Public examples

- `frame_checksum_longest_run(&[])` gives `0`.
- `frame_checksum_longest_run(&[0])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1379/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1379`.

[Module reading](https://doc.rust-lang.org/std/net/)
