# task-1839 — Shard occupancy · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Group the borrowed bytes into consecutive shards of 5, keeping the final partial shard. Use at most two threads to calculate one observation per shard, its nonzero byte count. Merge in input shard order. Do not copy the input to get a static lifetime.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use std::thread::scope and borrowed slices. During review, point to the operation that enforces its contract.

```rust
pub fn shard_occupancy_longest_run(input: &[u8]) -> u64
```

## Public examples

- `shard_occupancy_longest_run(&[])` gives `0`.
- `shard_occupancy_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1839/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1839`.

[Module reading](https://doc.rust-lang.org/std/thread/)
