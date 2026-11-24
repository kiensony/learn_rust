# task-2700 — Universal slice width · replay mismatch

Process consecutive borrowed chunks of up to 6 bytes. For each nonempty chunk observe the difference between greatest and least byte. Do not allocate a copy of the chunk. During review connect the lifetime of each view with the API that produced it.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use a higher-ranked bound for a projection callable on any temporary slice borrow. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn universal_slice_width_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `universal_slice_width_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[18])` gives `Some(1)`.
- `universal_slice_width_replay_mismatch(&[], &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2700/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2700`.

[Module reading](https://doc.rust-lang.org/reference/items/associated-items.html)
