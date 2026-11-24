# task-1824 — Shard masks · peaks

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Group the borrowed bytes into consecutive shards of 4, keeping the final partial shard. Use at most two threads to calculate one observation per shard, its bitwise OR. Merge in input shard order. Do not copy the input to get a static lifetime.

Keep the first observation and every later one strictly above all earlier observations. Use std::thread::scope and borrowed slices. During review, point to the operation that enforces its contract.

```rust
pub fn shard_masks_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `shard_masks_peaks(&[])` gives `vec![]`.
- `shard_masks_peaks(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1824/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1824`.

[Module reading](https://doc.rust-lang.org/std/thread/)
