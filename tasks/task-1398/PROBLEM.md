# task-1398 — Frame checksum · rolling load

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its payload XOR, starting at zero.

For each complete sliding window of width observations, return its sum as u64. Consecutive windows overlap by width minus one observations. Width zero and widths greater than the trace length yield an empty vector.

Use slice range checks before consuming a frame. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn frame_checksum_rolling_load(input: &[u8], width: usize) -> Vec<u64>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `frame_checksum_rolling_load(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2], 1)` gives `vec![4u64, 4u64, 0u64, 2u64]`.
- `frame_checksum_rolling_load(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1398/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1398`.

[Module reading](https://doc.rust-lang.org/std/net/)
