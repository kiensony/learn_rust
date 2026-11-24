# task-1837 — Shard occupancy · distinct

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Group the borrowed bytes into consecutive shards of 5, keeping the final partial shard. Use at most two threads to calculate one observation per shard, its nonzero byte count. Merge in input shard order. Do not copy the input to get a static lifetime.

Return distinct observations in ascending order. Use std::thread::scope and borrowed slices. During review, point to the operation that enforces its contract.

```rust
pub fn shard_occupancy_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `shard_occupancy_distinct(&[])` gives `vec![]`.
- `shard_occupancy_distinct(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1837/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1837`.

[Module reading](https://doc.rust-lang.org/std/thread/)
