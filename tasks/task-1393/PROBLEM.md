# task-1393 — Frame maxima · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its largest payload byte, or zero for an empty frame.

Return Some((minimum, maximum)) or None for an empty trace. Use slice range checks before consuming a frame. During review, point to the operation that enforces its contract.

```rust
pub fn frame_maxima_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `frame_maxima_bounds(&[])` gives `None`.
- `frame_maxima_bounds(&[0])` gives `Some((0u32, 0u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1393/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1393`.

[Module reading](https://doc.rust-lang.org/std/net/)
