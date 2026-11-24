# task-2062 — Double buffer receiver · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Drive a bounded Tokio mpsc channel of capacity 2 with deterministic input events. A nonzero byte is a try_send attempt. Count a full queue as a rejected send. A zero is a try_recv attempt. At end close the receiver and drain all buffered items. Observe each delivered byte exactly once in FIFO order.

Return the sum of all observations, widening each one before adding. Use tokio::sync::mpsc, receiver close and nonblocking operations. During review, point to the operation that enforces its contract.

```rust
pub fn double_buffer_receiver_total(input: &[u8]) -> u64
```

## Public examples

- `double_buffer_receiver_total(&[])` gives `0`.
- `double_buffer_receiver_total(&[4])` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2062/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2062`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
