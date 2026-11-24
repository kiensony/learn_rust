# task-1389 — First frame byte · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its first payload byte, or zero for an empty frame.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use slice range checks before consuming a frame. During review, point to the operation that enforces its contract.

```rust
pub fn first_frame_byte_longest_run(input: &[u8]) -> u64
```

## Public examples

- `first_frame_byte_longest_run(&[])` gives `0`.
- `first_frame_byte_longest_run(&[0])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1389/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1389`.

[Module reading](https://doc.rust-lang.org/std/net/)
