# task-2448 — Wide alignment · rolling load

Each byte denotes a logical offset, unrelated to an actual allocation address. Compute the smallest offset at or above it that meets a slot type's alignment. Define the slot with repr(align(8)), query its alignment, and widen before rounding. Observe each aligned offset.

For each complete sliding window of width observations, return its sum as u64. Consecutive windows overlap by width minus one observations. Width zero and widths greater than the trace length yield an empty vector.

Use repr(align) and std::mem::align_of. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn wide_alignment_rolling_load(input: &[u8], width: usize) -> Vec<u64>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `wide_alignment_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1)` gives `vec![8u64, 8u64, 16u64, 16u64, 24u64, 16u64, 0u64, 32u64, 8u64, 32u64]`.
- `wide_alignment_rolling_load(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2448/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2448`.

[Module reading](https://doc.rust-lang.org/reference/type-layout.html)
