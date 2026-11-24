# task-2967 — Loss accounting · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

A service with a bounded FIFO queue of capacity 3 receives one event per tick, starting at zero. A positive byte offers that payload, dropping it if full. A zero dequeues one item if available. Observe the payloads rejected by a full queue, in rejection order. Add a regression test combining repeated payloads and an empty dequeue. Explain how you would distinguish lost records from intentional rejection.

Return distinct observations in ascending order. Use a bounded FIFO state model and an explicit shutdown policy. During review, point to the operation that enforces its contract.

```rust
pub fn loss_accounting_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `loss_accounting_distinct(&[])` gives `vec![]`.
- `loss_accounting_distinct(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2967/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2967`.

[Module reading](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html)
