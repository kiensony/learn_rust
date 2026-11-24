# task-2097 — Double buffer receiver · run ledger

Drive a bounded Tokio mpsc channel of capacity 2 with deterministic input events. A nonzero byte is a try_send attempt. Count a full queue as a rejected send. A zero is a try_recv attempt. At end close the receiver and drain all buffered items. Observe each delivered byte exactly once in FIFO order.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use tokio::sync::mpsc, receiver close and nonblocking operations. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn double_buffer_receiver_run_ledger(input: &[u8]) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `double_buffer_receiver_run_ledger(&[9, 2, 2, 8, 0, 5])` gives `vec![(9u32, 1usize), (2u32, 1usize), (5u32, 1usize)]`.
- `double_buffer_receiver_run_ledger(&[])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2097/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2097`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
