# task-2808 — Service acknowledgments · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Inside a fresh current-thread runtime start one async consumer on a bounded channel of capacity two. Send every input byte in order, awaiting capacity. Close the sender and join the consumer, including for empty input. The consumer observes the accepted byte per delivery. Preserve FIFO order, use no sleeps and join every spawned task.

Return the absolute differences between each adjacent pair, in order. Use Tokio's current-thread runtime, mpsc::channel and an awaited JoinHandle. During review, point to the operation that enforces its contract.

```rust
pub fn service_acknowledgments_steps(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `service_acknowledgments_steps(&[])` gives `vec![]`.
- `service_acknowledgments_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2808/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2808`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
