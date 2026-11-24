# task-2850 — Service status · replay mismatch

Inside a fresh current-thread runtime start one async consumer on a bounded channel of capacity two. Send every input byte in order, awaiting capacity. Close the sender and join the consumer, including for empty input. The consumer observes one for payloads with their high bit set and zero otherwise per delivery. Preserve FIFO order, use no sleeps and join every spawned task.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use Tokio's current-thread runtime, mpsc::channel and an awaited JoinHandle. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn service_status_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `service_status_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[0, 0, 0, 0, 0, 0, 0, 0, 0])` gives `Some(9)`.
- `service_status_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2850/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2850`.

[Module reading](https://tokio.rs/tokio/topics/shutdown)
