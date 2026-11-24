# task-1384 — First frame byte · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its first payload byte, or zero for an empty frame.

Keep the first observation and every later one strictly above all earlier observations. Use slice range checks before consuming a frame. During review, point to the operation that enforces its contract.

```rust
pub fn first_frame_byte_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `first_frame_byte_peaks(&[])` gives `vec![]`.
- `first_frame_byte_peaks(&[0])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1384/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1384`.

[Module reading](https://doc.rust-lang.org/std/net/)
