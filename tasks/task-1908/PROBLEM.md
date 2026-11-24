# task-1908 — One slot queue · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Create a bounded FIFO channel of capacity 1. Each nonzero byte attempts to enqueue itself. Drop arrivals while full. Each zero requests exactly one nonblocking dequeue if available. At end disconnect the sender and drain. Observe delivered values in receive order.

Return the absolute differences between each adjacent pair, in order. Use std::sync::mpsc::sync_channel and nonblocking try operations. During review, point to the operation that enforces its contract.

```rust
pub fn one_slot_queue_steps(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `one_slot_queue_steps(&[])` gives `vec![]`.
- `one_slot_queue_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1908/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1908`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
