# task-1846 — Shard sums · threshold crossing

Group the borrowed bytes into consecutive shards of 2, keeping the final partial shard. Use at most two threads to calculate one observation per shard, its sum. Merge in input shard order. Do not copy the input to get a static lifetime.

Return the zero-based position and value of the first observation at least limit. Return None if no observation qualifies. The position refers to the derived sequence.

Use std::thread::scope and borrowed slices. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn shard_sums_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `shard_sums_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0)` gives `Some((0, 9))`.
- `shard_sums_threshold_crossing(&[], 0)` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1846/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1846`.

[Module reading](https://doc.rust-lang.org/std/thread/)
