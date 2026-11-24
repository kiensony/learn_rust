# task-1922 — Four slot queue · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Create a bounded FIFO channel of capacity 4. Each nonzero byte attempts to enqueue itself. Drop arrivals while full. Each zero requests exactly one nonblocking dequeue if available. At end disconnect the sender and drain. Observe delivered values in receive order.

Return the sum of all observations, widening each one before adding. Use std::sync::mpsc::sync_channel and nonblocking try operations. During review, point to the operation that enforces its contract.

```rust
pub fn four_slot_queue_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `four_slot_queue_total(&[])` gives `0`.
- `four_slot_queue_total(&[4])` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1922/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1922`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
