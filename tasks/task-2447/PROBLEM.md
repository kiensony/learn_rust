# task-2447 — Word alignment · run ledger

Each byte denotes a logical offset, unrelated to an actual allocation address. Compute the smallest offset at or above it that meets a slot type's alignment. Define the slot with repr(align(4)), query its alignment, and widen before rounding. Observe each aligned offset.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use repr(align) and std::mem::align_of. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn word_alignment_run_ledger(input: &[u8]) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `word_alignment_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31])` gives `vec![(4u32, 1usize), (8u32, 1usize), (12u32, 2usize), (24u32, 1usize), (16u32, 1usize), (0u32, 1usize), (28u32, 1usize), (8u32, 1usize), (32u32, 1usize)]`.
- `word_alignment_run_ledger(&[])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2447/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2447`.

[Module reading](https://doc.rust-lang.org/reference/type-layout.html)
