# task-2450 — Page alignment · replay mismatch

Each byte denotes a logical offset, unrelated to an actual allocation address. Compute the smallest offset at or above it that meets a slot type's alignment. Define the slot with repr(align(32)), query its alignment, and widen before rounding. Observe each aligned offset.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use repr(align) and std::mem::align_of. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn page_alignment_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `page_alignment_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[32, 32, 32, 32, 32, 32, 0, 32, 32])` gives `Some(9)`.
- `page_alignment_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2450/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2450`.

[Module reading](https://doc.rust-lang.org/reference/type-layout.html)
