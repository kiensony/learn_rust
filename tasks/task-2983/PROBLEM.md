# task-2983 — Service wait · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

A service with a bounded FIFO queue of capacity 5 receives one event per tick, starting at zero. A positive byte offers that payload, dropping it if full. A zero dequeues one item if available. Observe the number of event ticks each dequeued value spent in the queue; exclude undelivered entries. Add a regression test combining repeated payloads and an empty dequeue. Explain how you would distinguish lost records from intentional rejection.

Return Some((minimum, maximum)) or None for an empty trace. Use a bounded FIFO state model and an explicit shutdown policy. During review, point to the operation that enforces its contract.

```rust
pub fn service_wait_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `service_wait_bounds(&[])` gives `None`.
- `service_wait_bounds(&[4])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2983/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2983`.

[Module reading](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html)
