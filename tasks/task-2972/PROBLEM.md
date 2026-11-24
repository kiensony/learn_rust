# task-2972 — Queue depth · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

A service with a bounded FIFO queue of capacity 4 receives one event per tick, starting at zero. A positive byte offers that payload, dropping it if full. A zero dequeues one item if available. Observe queue length after every event. Add a regression test combining repeated payloads and an empty dequeue. Explain how you would distinguish lost records from intentional rejection.

Return the sum of all observations, widening each one before adding. Use a bounded FIFO state model and an explicit shutdown policy. During review, point to the operation that enforces its contract.

```rust
pub fn queue_depth_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `queue_depth_total(&[])` gives `0`.
- `queue_depth_total(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2972/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2972`.

[Module reading](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html)
