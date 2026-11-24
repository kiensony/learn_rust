# task-2094 — Five slot ingress · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Drive a bounded Tokio mpsc channel of capacity 5 with deterministic input events. A nonzero byte is a try_send attempt. Count a full queue as a rejected send. A zero is a try_recv attempt. At end close the receiver and drain all buffered items. Observe each delivered byte exactly once in FIFO order.

Keep the first observation and every later one strictly above all earlier observations. Use tokio::sync::mpsc, receiver close and nonblocking operations. During review, point to the operation that enforces its contract.

```rust
pub fn five_slot_ingress_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `five_slot_ingress_peaks(&[])` gives `vec![]`.
- `five_slot_ingress_peaks(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2094/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2094`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
