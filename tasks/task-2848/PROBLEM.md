# task-2848 — Service sharding · rolling load

Inside a fresh current-thread runtime start one async consumer on a bounded channel of capacity two. Send every input byte in order, awaiting capacity. Close the sender and join the consumer, including for empty input. The consumer observes the assigned shard ID (byte modulo four) per delivery. Preserve FIFO order, use no sleeps and join every spawned task.

For each complete sliding window of width observations, return its sum as u64. Consecutive windows overlap by width minus one observations. Width zero and widths greater than the trace length yield an empty vector.

Use Tokio's current-thread runtime, mpsc::channel and an awaited JoinHandle. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn service_sharding_rolling_load(input: &[u8], width: usize) -> Vec<u64>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `service_sharding_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1)` gives `vec![3u64, 2u64, 1u64, 0u64, 1u64, 3u64, 0u64, 2u64, 0u64, 3u64]`.
- `service_sharding_rolling_load(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2848/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2848`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
