# task-1395 — Frame maxima · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its largest payload byte, or zero for an empty frame.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use slice range checks before consuming a frame. During review, point to the operation that enforces its contract.

```rust
pub fn frame_maxima_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `frame_maxima_changes(&[])` gives `vec![]`.
- `frame_maxima_changes(&[0])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1395/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1395`.

[Module reading](https://doc.rust-lang.org/std/net/)
