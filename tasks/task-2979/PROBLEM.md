# task-2979 — Queue depth · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

A service with a bounded FIFO queue of capacity 4 receives one event per tick, starting at zero. A positive byte offers that payload, dropping it if full. A zero dequeues one item if available. Observe queue length after every event. Add a regression test combining repeated payloads and an empty dequeue. Explain how you would distinguish lost records from intentional rejection.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use a bounded FIFO state model and an explicit shutdown policy. During review, point to the operation that enforces its contract.

```rust
pub fn queue_depth_longest_run(input: &[u8]) -> u64
```

## Public examples

- `queue_depth_longest_run(&[])` gives `0`.
- `queue_depth_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2979/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2979`.

[Module reading](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html)
