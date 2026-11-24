# task-1848 — Shard masks · rolling load

Group the borrowed bytes into consecutive shards of 4, keeping the final partial shard. Use at most two threads to calculate one observation per shard, its bitwise OR. Merge in input shard order. Do not copy the input to get a static lifetime.

For each complete sliding window of width observations, return its sum as u64. Consecutive windows overlap by width minus one observations. Width zero and widths greater than the trace length yield an empty vector.

Use std::thread::scope and borrowed slices. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn shard_masks_rolling_load(input: &[u8], width: usize) -> Vec<u64>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `shard_masks_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1)` gives `vec![15u64, 31u64, 31u64]`.
- `shard_masks_rolling_load(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1848/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1848`.

[Module reading](https://doc.rust-lang.org/std/thread/)
