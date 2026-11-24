# task-1811 — Shard maxima · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Group the borrowed bytes into consecutive shards of 3, keeping the final partial shard. Use at most two threads to calculate one observation per shard, its maximum. Merge in input shard order. Do not copy the input to get a static lifetime.

Return the observations in input order, including duplicates. Use std::thread::scope and borrowed slices. During review, point to the operation that enforces its contract.

```rust
pub fn shard_maxima_trace(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shard_maxima_trace(&[])` gives `vec![]`.
- `shard_maxima_trace(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1811/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1811`.

[Module reading](https://doc.rust-lang.org/std/thread/)
