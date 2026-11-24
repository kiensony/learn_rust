# task-2075 — Three slot mailbox · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Drive a bounded Tokio mpsc channel of capacity 3 with deterministic input events. A nonzero byte is a try_send attempt. Count a full queue as a rejected send. A zero is a try_recv attempt. At end close the receiver and drain all buffered items. Observe each delivered byte exactly once in FIFO order.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use tokio::sync::mpsc, receiver close and nonblocking operations. During review, point to the operation that enforces its contract.

```rust
pub fn three_slot_mailbox_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `three_slot_mailbox_changes(&[])` gives `vec![]`.
- `three_slot_mailbox_changes(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2075/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2075`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
