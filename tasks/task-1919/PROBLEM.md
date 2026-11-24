# task-1919 — Two slot queue · longest run

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Create a bounded FIFO channel of capacity 2. Each nonzero byte attempts to enqueue itself. Drop arrivals while full. Each zero requests exactly one nonblocking dequeue if available. At end disconnect the sender and drain. Observe delivered values in receive order.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use std::sync::mpsc::sync_channel and nonblocking try operations. During review, point to the operation that enforces its contract.

```rust
pub fn two_slot_queue_longest_run(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `two_slot_queue_longest_run(&[])` gives `0`.
- `two_slot_queue_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1919/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1919`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
