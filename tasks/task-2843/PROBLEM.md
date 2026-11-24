# task-2843 — Service status · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Inside a fresh current-thread runtime start one async consumer on a bounded channel of capacity two. Send every input byte in order, awaiting capacity. Close the sender and join the consumer, including for empty input. The consumer observes one for payloads with their high bit set and zero otherwise per delivery. Preserve FIFO order, use no sleeps and join every spawned task.

Return Some((minimum, maximum)) or None for an empty trace. Use Tokio's current-thread runtime, mpsc::channel and an awaited JoinHandle. During review, point to the operation that enforces its contract.

```rust
pub fn service_status_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `service_status_bounds(&[])` gives `None`.
- `service_status_bounds(&[4])` gives `Some((0u32, 0u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2843/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2843`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
