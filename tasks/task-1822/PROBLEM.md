# task-1822 — Shard masks · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Group the borrowed bytes into consecutive shards of 4, keeping the final partial shard. Use at most two threads to calculate one observation per shard, its bitwise OR. Merge in input shard order. Do not copy the input to get a static lifetime.

Return the sum of all observations, widening each one before adding. Use std::thread::scope and borrowed slices. During review, point to the operation that enforces its contract.

```rust
pub fn shard_masks_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shard_masks_total(&[])` gives `0`.
- `shard_masks_total(&[4])` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1822/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1822`.

[Module reading](https://doc.rust-lang.org/std/thread/)
