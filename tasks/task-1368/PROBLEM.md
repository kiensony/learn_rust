# task-1368 — Frame offsets · steps

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its zero-based offset of the length byte.

Return the absolute differences between each adjacent pair, in order. Use slice range checks before consuming a frame. During review, point to the operation that enforces its contract.

```rust
pub fn frame_offsets_steps(input: &[u8]) -> Vec<u32>
```

## Public examples

- `frame_offsets_steps(&[])` gives `vec![]`.
- `frame_offsets_steps(&[0])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1368/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1368`.

[Module reading](https://doc.rust-lang.org/std/net/)
