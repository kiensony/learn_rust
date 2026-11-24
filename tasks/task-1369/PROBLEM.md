# task-1369 — Frame offsets · longest run

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its zero-based offset of the length byte.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use slice range checks before consuming a frame. During review, point to the operation that enforces its contract.

```rust
pub fn frame_offsets_longest_run(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `frame_offsets_longest_run(&[])` gives `0`.
- `frame_offsets_longest_run(&[0])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1369/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1369`.

[Module reading](https://doc.rust-lang.org/std/net/)
