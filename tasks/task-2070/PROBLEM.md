# task-2070 — Double buffer receiver · reverse

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Drive a bounded Tokio mpsc channel of capacity 2 with deterministic input events. A nonzero byte is a try_send attempt. Count a full queue as a rejected send. A zero is a try_recv attempt. At end close the receiver and drain all buffered items. Observe each delivered byte exactly once in FIFO order.

Return the observations in reverse order. Retain duplicate observations. Use tokio::sync::mpsc, receiver close and nonblocking operations. During review, point to the operation that enforces its contract.

```rust
pub fn double_buffer_receiver_reverse(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `double_buffer_receiver_reverse(&[])` gives `vec![]`.
- `double_buffer_receiver_reverse(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2070/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2070`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
