# task-2499 — Trailing raw reads · frequent readings

Advanced pointer-contract lab. Observe the final four bytes, or the whole input if shorter. For each visited offset construct the corresponding raw pointer with wrapping_add and check that it equals the address of the indexed byte. Keep access through the live slice. During review state which allocation owns each address and explain why a one-past pointer cannot be read.

Return up to keep distinct observations ordered by descending frequency, breaking ties by first appearance in the trace. Return an empty vector for keep zero. Each observation value appears at most once in the result.

Use slice::as_ptr, pointer equality, and safe indexed access. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn trailing_raw_reads_frequent_readings(input: &[u8], keep: usize) -> Vec<u32>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `trailing_raw_reads_frequent_readings(&[9, 2, 2, 8, 0, 5], 3)` gives `vec![2u32, 8u32, 0u32]`.
- `trailing_raw_reads_frequent_readings(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2499/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2499`.

[Module reading](https://doc.rust-lang.org/reference/unsafety.html)
