# task-2053 — Single slot receiver · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Drive a bounded Tokio mpsc channel of capacity 1 with deterministic input events. A nonzero byte is a try_send attempt. Count a full queue as a rejected send. A zero is a try_recv attempt. At end close the receiver and drain all buffered items. Observe each delivered byte exactly once in FIFO order.

Return Some((minimum, maximum)) or None for an empty trace. Use tokio::sync::mpsc, receiver close and nonblocking operations. During review, point to the operation that enforces its contract.

```rust
pub fn single_slot_receiver_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `single_slot_receiver_bounds(&[])` gives `None`.
- `single_slot_receiver_bounds(&[4])` gives `Some((4u32, 4u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2053/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2053`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
