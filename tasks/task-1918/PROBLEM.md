# task-1918 — Two slot queue · steps

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Create a bounded FIFO channel of capacity 2. Each nonzero byte attempts to enqueue itself. Drop arrivals while full. Each zero requests exactly one nonblocking dequeue if available. At end disconnect the sender and drain. Observe delivered values in receive order.

Return the absolute differences between each adjacent pair, in order. Use std::sync::mpsc::sync_channel and nonblocking try operations. During review, point to the operation that enforces its contract.

```rust
pub fn two_slot_queue_steps(input: &[u8]) -> Vec<u32>
```

## Public examples

- `two_slot_queue_steps(&[])` gives `vec![]`.
- `two_slot_queue_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1918/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1918`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
