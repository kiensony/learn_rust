# task-1394 — Frame maxima · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its largest payload byte, or zero for an empty frame.

Keep the first observation and every later one strictly above all earlier observations. Use slice range checks before consuming a frame. During review, point to the operation that enforces its contract.

```rust
pub fn frame_maxima_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `frame_maxima_peaks(&[])` gives `vec![]`.
- `frame_maxima_peaks(&[0])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1394/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1394`.

[Module reading](https://doc.rust-lang.org/std/net/)
