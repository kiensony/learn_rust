# task-2100 — Five slot ingress · replay mismatch

Drive a bounded Tokio mpsc channel of capacity 5 with deterministic input events. A nonzero byte is a try_send attempt. Count a full queue as a rejected send. A zero is a try_recv attempt. At end close the receiver and drain all buffered items. Observe each delivered byte exactly once in FIFO order.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use tokio::sync::mpsc, receiver close and nonblocking operations. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn five_slot_ingress_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `five_slot_ingress_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[3, 6, 9, 12, 21])` gives `Some(5)`.
- `five_slot_ingress_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2100/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2100`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
