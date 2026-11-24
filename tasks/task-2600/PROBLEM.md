# task-2600 — Interior stride proof · replay mismatch

Exercise a disjoint-mutation contract. Make an owned copy. Mutate the entries beginning at zero-based offset 2, advancing by 3. Increment each visited byte with wrapping arithmetic and observe its new value. Give each entry a unique temporary mutable reference. Run the tests under the course's pinned Miri toolchain and explain why no two live mutation sites alias.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use a safe mutable-slice iterator and Miri. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn interior_stride_proof_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `interior_stride_proof_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[10, 16])` gives `Some(2)`.
- `interior_stride_proof_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2600/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2600`.

[Module reading](https://github.com/rust-lang/miri)
