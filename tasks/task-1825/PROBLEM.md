# task-1825 — Shard masks · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Group the borrowed bytes into consecutive shards of 4, keeping the final partial shard. Use at most two threads to calculate one observation per shard, its bitwise OR. Merge in input shard order. Do not copy the input to get a static lifetime.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use std::thread::scope and borrowed slices. During review, point to the operation that enforces its contract.

```rust
pub fn shard_masks_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shard_masks_changes(&[])` gives `vec![]`.
- `shard_masks_changes(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1825/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1825`.

[Module reading](https://doc.rust-lang.org/std/thread/)
