# task-1946 — One slot queue · threshold crossing

Create a bounded FIFO channel of capacity 1. Each nonzero byte attempts to enqueue itself. Drop arrivals while full. Each zero requests exactly one nonblocking dequeue if available. At end disconnect the sender and drain. Observe delivered values in receive order.

Return the zero-based position and value of the first observation at least limit. Return None if no observation qualifies. The position refers to the derived sequence.

Use std::sync::mpsc::sync_channel and nonblocking try operations. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn one_slot_queue_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `one_slot_queue_threshold_crossing(&[9, 2, 2, 8, 0, 5], 0)` gives `Some((0, 9))`.
- `one_slot_queue_threshold_crossing(&[], 0)` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1946/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1946`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
