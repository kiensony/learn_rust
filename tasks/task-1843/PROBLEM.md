# task-1843 — Shard checksum · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Group the borrowed bytes into consecutive shards of 6, keeping the final partial shard. Use at most two threads to calculate one observation per shard, its bitwise XOR. Merge in input shard order. Do not copy the input to get a static lifetime.

Return Some((minimum, maximum)) or None for an empty trace. Use std::thread::scope and borrowed slices. During review, point to the operation that enforces its contract.

```rust
pub fn shard_checksum_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `shard_checksum_bounds(&[])` gives `None`.
- `shard_checksum_bounds(&[4])` gives `Some((4u32, 4u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1843/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1843`.

[Module reading](https://doc.rust-lang.org/std/thread/)
