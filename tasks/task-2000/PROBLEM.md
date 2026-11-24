# task-2000 — Atomic exchange · replay mismatch

Start an AtomicU32 at zero. For each byte perform swap with the byte widened to u32. Observe the value returned by the operation (the value preceding this update). Arithmetic wraps in u32. Explain why relaxed ordering suffices for this single-cell trace.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use AtomicU32 read-modify-write operations and Ordering::Relaxed. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn atomic_exchange_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `atomic_exchange_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[0, 3, 6, 9, 12, 21, 15, 0, 26])` gives `Some(9)`.
- `atomic_exchange_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2000/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2000`.

[Module reading](https://doc.rust-lang.org/std/sync/atomic/)
