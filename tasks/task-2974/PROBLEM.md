# task-2974 — Queue depth · peaks

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

A service with a bounded FIFO queue of capacity 4 receives one event per tick, starting at zero. A positive byte offers that payload, dropping it if full. A zero dequeues one item if available. Observe queue length after every event. Add a regression test combining repeated payloads and an empty dequeue. Explain how you would distinguish lost records from intentional rejection.

Keep the first observation and every later one strictly above all earlier observations. Use a bounded FIFO state model and an explicit shutdown policy. During review, point to the operation that enforces its contract.

```rust
pub fn queue_depth_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `queue_depth_peaks(&[])` gives `vec![]`.
- `queue_depth_peaks(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2974/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2974`.

[Module reading](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html)
