# task-2977 — Queue depth · distinct

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

A service with a bounded FIFO queue of capacity 4 receives one event per tick, starting at zero. A positive byte offers that payload, dropping it if full. A zero dequeues one item if available. Observe queue length after every event. Add a regression test combining repeated payloads and an empty dequeue. Explain how you would distinguish lost records from intentional rejection.

Return distinct observations in ascending order. Use a bounded FIFO state model and an explicit shutdown policy. During review, point to the operation that enforces its contract.

```rust
pub fn queue_depth_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `queue_depth_distinct(&[])` gives `vec![]`.
- `queue_depth_distinct(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2977/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2977`.

[Module reading](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html)
