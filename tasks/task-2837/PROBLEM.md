# task-2837 — Service expansion · distinct

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Inside a fresh current-thread runtime start one async consumer on a bounded channel of capacity two. Send every input byte in order, awaiting capacity. Close the sender and join the consumer, including for empty input. The consumer observes twice the byte widened to u32 per delivery. Preserve FIFO order, use no sleeps and join every spawned task.

Return distinct observations in ascending order. Use Tokio's current-thread runtime, mpsc::channel and an awaited JoinHandle. During review, point to the operation that enforces its contract.

```rust
pub fn service_expansion_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `service_expansion_distinct(&[])` gives `vec![]`.
- `service_expansion_distinct(&[4])` gives `vec![8u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2837/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2837`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
