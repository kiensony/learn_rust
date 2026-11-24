# task-2599 — Stride proof · frequent readings

Exercise a disjoint-mutation contract. Make an owned copy. Mutate the entries beginning at zero-based offset 1, advancing by 3. Increment each visited byte with wrapping arithmetic and observe its new value. Give each entry a unique temporary mutable reference. Run the tests under the course's pinned Miri toolchain and explain why no two live mutation sites alias.

Return up to keep distinct observations ordered by descending frequency, breaking ties by first appearance in the trace. Return an empty vector for keep zero. Each observation value appears at most once in the result.

Use a safe mutable-slice iterator and Miri. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn stride_proof_frequent_readings(input: &[u8], keep: usize) -> Vec<u32>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `stride_proof_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3)` gives `vec![7u32, 22u32, 27u32]`.
- `stride_proof_frequent_readings(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2599/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2599`.

[Module reading](https://github.com/rust-lang/miri)
