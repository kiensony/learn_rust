# task-2449 — Block alignment · frequent readings

Each byte denotes a logical offset, unrelated to an actual allocation address. Compute the smallest offset at or above it that meets a slot type's alignment. Define the slot with repr(align(16)), query its alignment, and widen before rounding. Observe each aligned offset.

Return up to keep distinct observations ordered by descending frequency, breaking ties by first appearance in the trace. Return an empty vector for keep zero. Each observation value appears at most once in the result.

Use repr(align) and std::mem::align_of. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn block_alignment_frequent_readings(input: &[u8], keep: usize) -> Vec<u32>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `block_alignment_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3)` gives `vec![16u32, 32u32, 0u32]`.
- `block_alignment_frequent_readings(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2449/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2449`.

[Module reading](https://doc.rust-lang.org/reference/type-layout.html)
