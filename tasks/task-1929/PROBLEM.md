# task-1929 — Four slot queue · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Create a bounded FIFO channel of capacity 4. Each nonzero byte attempts to enqueue itself. Drop arrivals while full. Each zero requests exactly one nonblocking dequeue if available. At end disconnect the sender and drain. Observe delivered values in receive order.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use std::sync::mpsc::sync_channel and nonblocking try operations. During review, point to the operation that enforces its contract.

```rust
pub fn four_slot_queue_longest_run(input: &[u8]) -> u64
```

## Public examples

- `four_slot_queue_longest_run(&[])` gives `0`.
- `four_slot_queue_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1929/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1929`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
