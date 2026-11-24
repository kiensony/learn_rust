# task-1924 — Four slot queue · peaks

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Create a bounded FIFO channel of capacity 4. Each nonzero byte attempts to enqueue itself. Drop arrivals while full. Each zero requests exactly one nonblocking dequeue if available. At end disconnect the sender and drain. Observe delivered values in receive order.

Keep the first observation and every later one strictly above all earlier observations. Use std::sync::mpsc::sync_channel and nonblocking try operations. During review, point to the operation that enforces its contract.

```rust
pub fn four_slot_queue_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `four_slot_queue_peaks(&[])` gives `vec![]`.
- `four_slot_queue_peaks(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1924/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1924`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
