# task-2993 — Drain order · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

A service with a bounded FIFO queue of capacity 6 receives one event per tick, starting at zero. A positive byte offers that payload, dropping it if full. A zero dequeues one item if available. Observe only the pending values at end, in FIFO drain order. Add a regression test combining repeated payloads and an empty dequeue. Explain how you would distinguish lost records from intentional rejection.

Return Some((minimum, maximum)) or None for an empty trace. Use a bounded FIFO state model and an explicit shutdown policy. During review, point to the operation that enforces its contract.

```rust
pub fn drain_order_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `drain_order_bounds(&[])` gives `None`.
- `drain_order_bounds(&[4])` gives `Some((4u32, 4u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2993/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2993`.

[Module reading](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html)
