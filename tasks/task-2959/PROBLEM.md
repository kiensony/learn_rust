# task-2959 — Journal replay · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

A service with a bounded FIFO queue of capacity 2 receives one event per tick, starting at zero. A positive byte offers that payload, dropping it if full. A zero dequeues one item if available. Observe the successfully dequeued values, followed by a FIFO drain of the remaining queue. Add a regression test combining repeated payloads and an empty dequeue. Explain how you would distinguish lost records from intentional rejection.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use a bounded FIFO state model and an explicit shutdown policy. During review, point to the operation that enforces its contract.

```rust
pub fn journal_replay_longest_run(input: &[u8]) -> u64
```

## Public examples

- `journal_replay_longest_run(&[])` gives `0`.
- `journal_replay_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2959/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2959`.

[Module reading](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html)
