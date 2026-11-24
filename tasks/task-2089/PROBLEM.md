# task-2089 — Four slot intake · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Drive a bounded Tokio mpsc channel of capacity 4 with deterministic input events. A nonzero byte is a try_send attempt. Count a full queue as a rejected send. A zero is a try_recv attempt. At end close the receiver and drain all buffered items. Observe each delivered byte exactly once in FIFO order.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use tokio::sync::mpsc, receiver close and nonblocking operations. During review, point to the operation that enforces its contract.

```rust
pub fn four_slot_intake_longest_run(input: &[u8]) -> u64
```

## Public examples

- `four_slot_intake_longest_run(&[])` gives `0`.
- `four_slot_intake_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2089/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2089`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
