# task-2052 — Single slot receiver · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Drive a bounded Tokio mpsc channel of capacity 1 with deterministic input events. A nonzero byte is a try_send attempt. Count a full queue as a rejected send. A zero is a try_recv attempt. At end close the receiver and drain all buffered items. Observe each delivered byte exactly once in FIFO order.

Return the sum of all observations, widening each one before adding. Use tokio::sync::mpsc, receiver close and nonblocking operations. During review, point to the operation that enforces its contract.

```rust
pub fn single_slot_receiver_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `single_slot_receiver_total(&[])` gives `0`.
- `single_slot_receiver_total(&[4])` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2052/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2052`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
