# task-1353 — Frame lengths · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its payload length.

Return Some((minimum, maximum)) or None for an empty trace. Use slice range checks before consuming a frame. During review, point to the operation that enforces its contract.

```rust
pub fn frame_lengths_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `frame_lengths_bounds(&[])` gives `None`.
- `frame_lengths_bounds(&[0])` gives `Some((0u32, 0u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1353/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1353`.

[Module reading](https://doc.rust-lang.org/std/net/)
