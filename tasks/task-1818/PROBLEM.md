# task-1818 — Shard maxima · steps

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Group the borrowed bytes into consecutive shards of 3, keeping the final partial shard. Use at most two threads to calculate one observation per shard, its maximum. Merge in input shard order. Do not copy the input to get a static lifetime.

Return the absolute differences between each adjacent pair, in order. Use std::thread::scope and borrowed slices. During review, point to the operation that enforces its contract.

```rust
pub fn shard_maxima_steps(input: &[u8]) -> Vec<u32>
```

## Public examples

- `shard_maxima_steps(&[])` gives `vec![]`.
- `shard_maxima_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1818/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1818`.

[Module reading](https://doc.rust-lang.org/std/thread/)
