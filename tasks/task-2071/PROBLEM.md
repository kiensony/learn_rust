# task-2071 — Three slot mailbox · trace

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Drive a bounded Tokio mpsc channel of capacity 3 with deterministic input events. A nonzero byte is a try_send attempt. Count a full queue as a rejected send. A zero is a try_recv attempt. At end close the receiver and drain all buffered items. Observe each delivered byte exactly once in FIFO order.

Return the observations in input order, including duplicates. Use tokio::sync::mpsc, receiver close and nonblocking operations. During review, point to the operation that enforces its contract.

```rust
pub fn three_slot_mailbox_trace(input: &[u8]) -> Vec<u32>
```

## Public examples

- `three_slot_mailbox_trace(&[])` gives `vec![]`.
- `three_slot_mailbox_trace(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2071/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2071`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
