# task-2994 — Drain order · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

A service with a bounded FIFO queue of capacity 6 receives one event per tick, starting at zero. A positive byte offers that payload, dropping it if full. A zero dequeues one item if available. Observe only the pending values at end, in FIFO drain order. Add a regression test combining repeated payloads and an empty dequeue. Explain how you would distinguish lost records from intentional rejection.

Keep the first observation and every later one strictly above all earlier observations. Use a bounded FIFO state model and an explicit shutdown policy. During review, point to the operation that enforces its contract.

```rust
pub fn drain_order_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `drain_order_peaks(&[])` gives `vec![]`.
- `drain_order_peaks(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2994/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2994`.

[Module reading](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html)
