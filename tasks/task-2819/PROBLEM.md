# task-2819 — Service validation · longest run

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Inside a fresh current-thread runtime start one async consumer on a bounded channel of capacity two. Send every input byte in order, awaiting capacity. Close the sender and join the consumer, including for empty input. The consumer observes one for nonzero payloads and zero for zero per delivery. Preserve FIFO order, use no sleeps and join every spawned task.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use Tokio's current-thread runtime, mpsc::channel and an awaited JoinHandle. During review, point to the operation that enforces its contract.

```rust
pub fn service_validation_longest_run(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `service_validation_longest_run(&[])` gives `0`.
- `service_validation_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2819/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2819`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
