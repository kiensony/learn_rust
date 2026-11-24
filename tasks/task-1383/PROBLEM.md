# task-1383 — First frame byte · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its first payload byte, or zero for an empty frame.

Return Some((minimum, maximum)) or None for an empty trace. Use slice range checks before consuming a frame. During review, point to the operation that enforces its contract.

```rust
pub fn first_frame_byte_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `first_frame_byte_bounds(&[])` gives `None`.
- `first_frame_byte_bounds(&[0])` gives `Some((0u32, 0u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1383/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1383`.

[Module reading](https://doc.rust-lang.org/std/net/)
