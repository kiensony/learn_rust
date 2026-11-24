# task-1827 — Shard masks · distinct

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Group the borrowed bytes into consecutive shards of 4, keeping the final partial shard. Use at most two threads to calculate one observation per shard, its bitwise OR. Merge in input shard order. Do not copy the input to get a static lifetime.

Return distinct observations in ascending order. Use std::thread::scope and borrowed slices. During review, point to the operation that enforces its contract.

```rust
pub fn shard_masks_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `shard_masks_distinct(&[])` gives `vec![]`.
- `shard_masks_distinct(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1827/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1827`.

[Module reading](https://doc.rust-lang.org/std/thread/)
