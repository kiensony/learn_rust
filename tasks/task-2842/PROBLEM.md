# task-2842 — Service status · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Inside a fresh current-thread runtime start one async consumer on a bounded channel of capacity two. Send every input byte in order, awaiting capacity. Close the sender and join the consumer, including for empty input. The consumer observes one for payloads with their high bit set and zero otherwise per delivery. Preserve FIFO order, use no sleeps and join every spawned task.

Return the sum of all observations, widening each one before adding. Use Tokio's current-thread runtime, mpsc::channel and an awaited JoinHandle. During review, point to the operation that enforces its contract.

```rust
pub fn service_status_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `service_status_total(&[])` gives `0`.
- `service_status_total(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2842/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2842`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
