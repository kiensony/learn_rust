# task-2807 — Service acknowledgments · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Inside a fresh current-thread runtime start one async consumer on a bounded channel of capacity two. Send every input byte in order, awaiting capacity. Close the sender and join the consumer, including for empty input. The consumer observes the accepted byte per delivery. Preserve FIFO order, use no sleeps and join every spawned task.

Return distinct observations in ascending order. Use Tokio's current-thread runtime, mpsc::channel and an awaited JoinHandle. During review, point to the operation that enforces its contract.

```rust
pub fn service_acknowledgments_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `service_acknowledgments_distinct(&[])` gives `vec![]`.
- `service_acknowledgments_distinct(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2807/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2807`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
