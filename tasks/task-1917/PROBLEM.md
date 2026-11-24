# task-1917 — Two slot queue · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Create a bounded FIFO channel of capacity 2. Each nonzero byte attempts to enqueue itself. Drop arrivals while full. Each zero requests exactly one nonblocking dequeue if available. At end disconnect the sender and drain. Observe delivered values in receive order.

Return distinct observations in ascending order. Use std::sync::mpsc::sync_channel and nonblocking try operations. During review, point to the operation that enforces its contract.

```rust
pub fn two_slot_queue_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `two_slot_queue_distinct(&[])` gives `vec![]`.
- `two_slot_queue_distinct(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1917/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1917`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
