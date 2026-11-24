# task-1925 — Four slot queue · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Create a bounded FIFO channel of capacity 4. Each nonzero byte attempts to enqueue itself. Drop arrivals while full. Each zero requests exactly one nonblocking dequeue if available. At end disconnect the sender and drain. Observe delivered values in receive order.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use std::sync::mpsc::sync_channel and nonblocking try operations. During review, point to the operation that enforces its contract.

```rust
pub fn four_slot_queue_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `four_slot_queue_changes(&[])` gives `vec![]`.
- `four_slot_queue_changes(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1925/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1925`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
