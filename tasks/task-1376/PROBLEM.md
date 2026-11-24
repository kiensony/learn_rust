# task-1376 — Frame checksum · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its payload XOR, starting at zero.

Count adjacent pairs for which the second observation is strictly smaller. Use slice range checks before consuming a frame. During review, point to the operation that enforces its contract.

```rust
pub fn frame_checksum_descending_steps(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `frame_checksum_descending_steps(&[])` gives `0`.
- `frame_checksum_descending_steps(&[0])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1376/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1376`.

[Module reading](https://doc.rust-lang.org/std/net/)
