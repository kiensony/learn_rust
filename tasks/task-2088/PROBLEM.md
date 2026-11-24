# task-2088 — Four slot intake · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Drive a bounded Tokio mpsc channel of capacity 4 with deterministic input events. A nonzero byte is a try_send attempt. Count a full queue as a rejected send. A zero is a try_recv attempt. At end close the receiver and drain all buffered items. Observe each delivered byte exactly once in FIFO order.

Return the absolute differences between each adjacent pair, in order. Use tokio::sync::mpsc, receiver close and nonblocking operations. During review, point to the operation that enforces its contract.

```rust
pub fn four_slot_intake_steps(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `four_slot_intake_steps(&[])` gives `vec![]`.
- `four_slot_intake_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2088/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2088`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
