# task-2984 — Service wait · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

A service with a bounded FIFO queue of capacity 5 receives one event per tick, starting at zero. A positive byte offers that payload, dropping it if full. A zero dequeues one item if available. Observe the number of event ticks each dequeued value spent in the queue; exclude undelivered entries. Add a regression test combining repeated payloads and an empty dequeue. Explain how you would distinguish lost records from intentional rejection.

Keep the first observation and every later one strictly above all earlier observations. Use a bounded FIFO state model and an explicit shutdown policy. During review, point to the operation that enforces its contract.

```rust
pub fn service_wait_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `service_wait_peaks(&[])` gives `vec![]`.
- `service_wait_peaks(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2984/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2984`.

[Module reading](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html)
