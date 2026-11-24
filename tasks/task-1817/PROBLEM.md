# task-1817 — Shard maxima · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Group the borrowed bytes into consecutive shards of 3, keeping the final partial shard. Use at most two threads to calculate one observation per shard, its maximum. Merge in input shard order. Do not copy the input to get a static lifetime.

Return distinct observations in ascending order. Use std::thread::scope and borrowed slices. During review, point to the operation that enforces its contract.

```rust
pub fn shard_maxima_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `shard_maxima_distinct(&[])` gives `vec![]`.
- `shard_maxima_distinct(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1817/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1817`.

[Module reading](https://doc.rust-lang.org/std/thread/)
