# task-1358 — Frame lengths · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its payload length.

Return the absolute differences between each adjacent pair, in order. Use slice range checks before consuming a frame. During review, point to the operation that enforces its contract.

```rust
pub fn frame_lengths_steps(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `frame_lengths_steps(&[])` gives `vec![]`.
- `frame_lengths_steps(&[0])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1358/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1358`.

[Module reading](https://doc.rust-lang.org/std/net/)
