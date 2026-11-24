# task-1364 — Frame offsets · peaks

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its zero-based offset of the length byte.

Keep the first observation and every later one strictly above all earlier observations. Use slice range checks before consuming a frame. During review, point to the operation that enforces its contract.

```rust
pub fn frame_offsets_peaks(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `frame_offsets_peaks(&[])` gives `vec![]`.
- `frame_offsets_peaks(&[0])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1364/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1364`.

[Module reading](https://doc.rust-lang.org/std/net/)
