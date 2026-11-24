# task-2699 — Universal slice sum · frequent readings

Process consecutive borrowed chunks of up to 5 bytes. For each nonempty chunk observe the sum of bytes. Do not allocate a copy of the chunk. During review connect the lifetime of each view with the API that produced it.

Return up to keep distinct observations ordered by descending frequency, breaking ties by first appearance in the trace. Return an empty vector for keep zero. Each observation value appears at most once in the result.

Use a higher-ranked bound for a projection callable on any temporary slice borrow. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn universal_slice_sum_frequent_readings(input: &[u8], keep: usize) -> Vec<u32>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `universal_slice_sum_frequent_readings(&[9, 2, 2, 8, 0, 5], 3)` gives `vec![21u32, 5u32]`.
- `universal_slice_sum_frequent_readings(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2699/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2699`.

[Module reading](https://doc.rust-lang.org/reference/items/associated-items.html)
