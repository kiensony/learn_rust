# task-2829 — Service sharding · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Inside a fresh current-thread runtime start one async consumer on a bounded channel of capacity two. Send every input byte in order, awaiting capacity. Close the sender and join the consumer, including for empty input. The consumer observes the assigned shard ID (byte modulo four) per delivery. Preserve FIFO order, use no sleeps and join every spawned task.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use Tokio's current-thread runtime, mpsc::channel and an awaited JoinHandle. During review, point to the operation that enforces its contract.

```rust
pub fn service_sharding_longest_run(input: &[u8]) -> u64
```

## Public examples

- `service_sharding_longest_run(&[])` gives `0`.
- `service_sharding_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2829/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2829`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
