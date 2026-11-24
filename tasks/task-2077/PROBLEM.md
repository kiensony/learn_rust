# task-2077 — Cancellation leaves a reserved channel intact

Fill a channel of capacity one with first. Poll a reservation once while it is full and cancel the pending reservation by dropping its future. Then deliver the original queued item, admit second, close the channel and drain. Return every delivered byte exactly once, in order, including duplicates. Add a regression in which both bytes are equal and show why output length matters. Use no sleeps or timeouts.

```rust
pub fn cancel_reservation(first: u8, second: u8) -> Vec<u8>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `cancel_reservation(4, 9)` gives `vec![4,9]`.
- `cancel_reservation(1, 1)` gives `vec![1,1]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2077/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2077`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
