# task-1385 — First frame byte · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its first payload byte, or zero for an empty frame.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use slice range checks before consuming a frame. During review, point to the operation that enforces its contract.

```rust
pub fn first_frame_byte_changes(input: &[u8]) -> Vec<u32>
```

## Public examples

- `first_frame_byte_changes(&[])` gives `vec![]`.
- `first_frame_byte_changes(&[0])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1385/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1385`.

[Module reading](https://doc.rust-lang.org/std/net/)
