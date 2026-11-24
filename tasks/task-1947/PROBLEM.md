# task-1947 — Two slot queue · run ledger

Create a bounded FIFO channel of capacity 2. Each nonzero byte attempts to enqueue itself. Drop arrivals while full. Each zero requests exactly one nonblocking dequeue if available. At end disconnect the sender and drain. Observe delivered values in receive order.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use std::sync::mpsc::sync_channel and nonblocking try operations. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn two_slot_queue_run_ledger(input: &[u8]) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `two_slot_queue_run_ledger(&[9, 2, 2, 8, 0, 5])` gives `vec![(9u32, 1usize), (2u32, 1usize), (5u32, 1usize)]`.
- `two_slot_queue_run_ledger(&[])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1947/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1947`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
