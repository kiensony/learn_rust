# task-2500 — Interior raw reads · replay mismatch

Advanced pointer-contract lab. Observe all bytes except the first and last; an input shorter than three yields no bytes. For each visited offset construct the corresponding raw pointer with wrapping_add and check that it equals the address of the indexed byte. Keep access through the live slice. During review state which allocation owns each address and explain why a one-past pointer cannot be read.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use slice::as_ptr, pointer equality, and safe indexed access. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn interior_raw_reads_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `interior_raw_reads_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[6, 9, 12, 21, 15, 0, 26])` gives `Some(7)`.
- `interior_raw_reads_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2500/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2500`.

[Module reading](https://doc.rust-lang.org/reference/unsafety.html)
