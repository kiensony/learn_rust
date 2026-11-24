# task-1367 — Frame offsets · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its zero-based offset of the length byte.

Return distinct observations in ascending order. Use slice range checks before consuming a frame. During review, point to the operation that enforces its contract.

```rust
pub fn frame_offsets_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `frame_offsets_distinct(&[])` gives `vec![]`.
- `frame_offsets_distinct(&[0])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1367/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1367`.

[Module reading](https://doc.rust-lang.org/std/net/)
