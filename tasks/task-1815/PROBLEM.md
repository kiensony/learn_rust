# task-1815 — Shard maxima · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Group the borrowed bytes into consecutive shards of 3, keeping the final partial shard. Use at most two threads to calculate one observation per shard, its maximum. Merge in input shard order. Do not copy the input to get a static lifetime.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use std::thread::scope and borrowed slices. During review, point to the operation that enforces its contract.

```rust
pub fn shard_maxima_changes(input: &[u8]) -> Vec<u32>
```

## Public examples

- `shard_maxima_changes(&[])` gives `vec![]`.
- `shard_maxima_changes(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1815/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1815`.

[Module reading](https://doc.rust-lang.org/std/thread/)
