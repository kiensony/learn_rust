# task-1948 — Four slot queue · rolling load

Create a bounded FIFO channel of capacity 4. Each nonzero byte attempts to enqueue itself. Drop arrivals while full. Each zero requests exactly one nonblocking dequeue if available. At end disconnect the sender and drain. Observe delivered values in receive order.

For each complete sliding window of width observations, return its sum as u64. Consecutive windows overlap by width minus one observations. Width zero and widths greater than the trace length yield an empty vector.

Use std::sync::mpsc::sync_channel and nonblocking try operations. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn four_slot_queue_rolling_load(input: &[u8], width: usize) -> Vec<u64>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `four_slot_queue_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1)` gives `vec![3u64, 6u64, 9u64, 12u64, 26u64]`.
- `four_slot_queue_rolling_load(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1948/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1948`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
