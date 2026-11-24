# task-2960 — Journal replay · reverse

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

A service with a bounded FIFO queue of capacity 2 receives one event per tick, starting at zero. A positive byte offers that payload, dropping it if full. A zero dequeues one item if available. Observe the successfully dequeued values, followed by a FIFO drain of the remaining queue. Add a regression test combining repeated payloads and an empty dequeue. Explain how you would distinguish lost records from intentional rejection.

Return the observations in reverse order. Retain duplicate observations. Use a bounded FIFO state model and an explicit shutdown policy. During review, point to the operation that enforces its contract.

```rust
pub fn journal_replay_reverse(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `journal_replay_reverse(&[])` gives `vec![]`.
- `journal_replay_reverse(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2960/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2960`.

[Module reading](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html)
