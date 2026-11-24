# task-1374 — Frame checksum · peaks

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its payload XOR, starting at zero.

Keep the first observation and every later one strictly above all earlier observations. Use slice range checks before consuming a frame. During review, point to the operation that enforces its contract.

```rust
pub fn frame_checksum_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `frame_checksum_peaks(&[])` gives `vec![]`.
- `frame_checksum_peaks(&[0])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1374/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1374`.

[Module reading](https://doc.rust-lang.org/std/net/)
