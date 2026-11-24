# task-1362 — Frame offsets · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its zero-based offset of the length byte.

Return the sum of all observations, widening each one before adding. Use slice range checks before consuming a frame. During review, point to the operation that enforces its contract.

```rust
pub fn frame_offsets_total(input: &[u8]) -> u64
```

## Public examples

- `frame_offsets_total(&[])` gives `0`.
- `frame_offsets_total(&[0])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1362/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1362`.

[Module reading](https://doc.rust-lang.org/std/net/)
