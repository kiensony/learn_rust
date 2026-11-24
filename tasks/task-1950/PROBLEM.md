# task-1950 — Locked batch peaks · replay mismatch

Publish one maximum for each consecutive nonempty chunk of 3 bytes into a mutex-protected journal. Compute each result before acquiring the lock. Observe the journal after ownership returns to the caller.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use std::sync::Mutex with a short critical section. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn locked_batch_peaks_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `locked_batch_peaks_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[9, 21, 26])` gives `Some(3)`.
- `locked_batch_peaks_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1950/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1950`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
