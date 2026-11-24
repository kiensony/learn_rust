# task-1819 — Shard maxima · longest run

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Group the borrowed bytes into consecutive shards of 3, keeping the final partial shard. Use at most two threads to calculate one observation per shard, its maximum. Merge in input shard order. Do not copy the input to get a static lifetime.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use std::thread::scope and borrowed slices. During review, point to the operation that enforces its contract.

```rust
pub fn shard_maxima_longest_run(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shard_maxima_longest_run(&[])` gives `0`.
- `shard_maxima_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1819/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1819`.

[Module reading](https://doc.rust-lang.org/std/thread/)
