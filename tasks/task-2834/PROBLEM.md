# task-2834 — Service expansion · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Inside a fresh current-thread runtime start one async consumer on a bounded channel of capacity two. Send every input byte in order, awaiting capacity. Close the sender and join the consumer, including for empty input. The consumer observes twice the byte widened to u32 per delivery. Preserve FIFO order, use no sleeps and join every spawned task.

Keep the first observation and every later one strictly above all earlier observations. Use Tokio's current-thread runtime, mpsc::channel and an awaited JoinHandle. During review, point to the operation that enforces its contract.

```rust
pub fn service_expansion_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `service_expansion_peaks(&[])` gives `vec![]`.
- `service_expansion_peaks(&[4])` gives `vec![8u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2834/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2834`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
