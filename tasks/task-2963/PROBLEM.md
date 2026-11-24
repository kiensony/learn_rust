# task-2963 — Loss accounting · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

A service with a bounded FIFO queue of capacity 3 receives one event per tick, starting at zero. A positive byte offers that payload, dropping it if full. A zero dequeues one item if available. Observe the payloads rejected by a full queue, in rejection order. Add a regression test combining repeated payloads and an empty dequeue. Explain how you would distinguish lost records from intentional rejection.

Return Some((minimum, maximum)) or None for an empty trace. Use a bounded FIFO state model and an explicit shutdown policy. During review, point to the operation that enforces its contract.

```rust
pub fn loss_accounting_bounds(input: &[u8]) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `loss_accounting_bounds(&[])` gives `None`.
- `loss_accounting_bounds(&[4])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2963/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2963`.

[Module reading](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html)
