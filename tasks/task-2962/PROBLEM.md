# task-2962 — Loss accounting · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

A service with a bounded FIFO queue of capacity 3 receives one event per tick, starting at zero. A positive byte offers that payload, dropping it if full. A zero dequeues one item if available. Observe the payloads rejected by a full queue, in rejection order. Add a regression test combining repeated payloads and an empty dequeue. Explain how you would distinguish lost records from intentional rejection.

Return the sum of all observations, widening each one before adding. Use a bounded FIFO state model and an explicit shutdown policy. During review, point to the operation that enforces its contract.

```rust
pub fn loss_accounting_total(input: &[u8]) -> u64
```

## Public examples

- `loss_accounting_total(&[])` gives `0`.
- `loss_accounting_total(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2962/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2962`.

[Module reading](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html)
