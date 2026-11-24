# task-1365 — Frame offsets · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its zero-based offset of the length byte.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use slice range checks before consuming a frame. During review, point to the operation that enforces its contract.

```rust
pub fn frame_offsets_changes(input: &[u8]) -> Vec<u32>
```

## Public examples

- `frame_offsets_changes(&[])` gives `vec![]`.
- `frame_offsets_changes(&[0])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1365/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1365`.

[Module reading](https://doc.rust-lang.org/std/net/)
