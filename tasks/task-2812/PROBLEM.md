# task-2812 — Service validation · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Inside a fresh current-thread runtime start one async consumer on a bounded channel of capacity two. Send every input byte in order, awaiting capacity. Close the sender and join the consumer, including for empty input. The consumer observes one for nonzero payloads and zero for zero per delivery. Preserve FIFO order, use no sleeps and join every spawned task.

Return the sum of all observations, widening each one before adding. Use Tokio's current-thread runtime, mpsc::channel and an awaited JoinHandle. During review, point to the operation that enforces its contract.

```rust
pub fn service_validation_total(input: &[u8]) -> u64
```

## Public examples

- `service_validation_total(&[])` gives `0`.
- `service_validation_total(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2812/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2812`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
