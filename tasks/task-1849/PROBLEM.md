# task-1849 — Shard occupancy · frequent readings

Group the borrowed bytes into consecutive shards of 5, keeping the final partial shard. Use at most two threads to calculate one observation per shard, its nonzero byte count. Merge in input shard order. Do not copy the input to get a static lifetime.

Return up to keep distinct observations ordered by descending frequency, breaking ties by first appearance in the trace. Return an empty vector for keep zero. Each observation value appears at most once in the result.

Use std::thread::scope and borrowed slices. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn shard_occupancy_frequent_readings(input: &[u8], keep: usize) -> Vec<u32>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `shard_occupancy_frequent_readings(&[9, 2, 2, 8, 0, 5], 3)` gives `vec![4u32, 1u32]`.
- `shard_occupancy_frequent_readings(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1849/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1849`.

[Module reading](https://doc.rust-lang.org/std/thread/)
