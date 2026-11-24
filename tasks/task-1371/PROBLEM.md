# task-1371 — Frame checksum · trace

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its payload XOR, starting at zero.

Return the observations in input order, including duplicates. Use slice range checks before consuming a frame. During review, point to the operation that enforces its contract.

```rust
pub fn frame_checksum_trace(input: &[u8]) -> Vec<u32>
```

## Public examples

- `frame_checksum_trace(&[])` gives `vec![]`.
- `frame_checksum_trace(&[0])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1371/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1371`.

[Module reading](https://doc.rust-lang.org/std/net/)
