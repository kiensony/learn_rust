# task-1382 — First frame byte · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its first payload byte, or zero for an empty frame.

Return the sum of all observations, widening each one before adding. Use slice range checks before consuming a frame. During review, point to the operation that enforces its contract.

```rust
pub fn first_frame_byte_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `first_frame_byte_total(&[])` gives `0`.
- `first_frame_byte_total(&[0])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1382/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1382`.

[Module reading](https://doc.rust-lang.org/std/net/)
