# task-1909 — One slot queue · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Create a bounded FIFO channel of capacity 1. Each nonzero byte attempts to enqueue itself. Drop arrivals while full. Each zero requests exactly one nonblocking dequeue if available. At end disconnect the sender and drain. Observe delivered values in receive order.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use std::sync::mpsc::sync_channel and nonblocking try operations. During review, point to the operation that enforces its contract.

```rust
pub fn one_slot_queue_longest_run(input: &[u8]) -> u64
```

## Public examples

- `one_slot_queue_longest_run(&[])` gives `0`.
- `one_slot_queue_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1909/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1909`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
