# task-1850 — Shard checksum · replay mismatch

Group the borrowed bytes into consecutive shards of 6, keeping the final partial shard. Use at most two threads to calculate one observation per shard, its bitwise XOR. Merge in input shard order. Do not copy the input to get a static lifetime.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use std::thread::scope and borrowed slices. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn shard_checksum_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `shard_checksum_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[26])` gives `Some(1)`.
- `shard_checksum_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1850/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1850`.

[Module reading](https://doc.rust-lang.org/std/thread/)
