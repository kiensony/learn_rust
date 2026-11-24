# task-1649 — Clamp property · frequent readings

Each byte is a generated test input. Observe one when the following predicate holds and zero otherwise: clamping to [4, 16] changes the value. As a separate regression test, enumerate the full u8 domain; check the property against an independently computed model and report the smallest counterexample.

Return up to keep distinct observations ordered by descending frequency, breaking ties by first appearance in the trace. Return an empty vector for keep zero. Each observation value appears at most once in the result.

Use an exhaustive u8 domain test with a model separate from the implementation. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn clamp_property_frequent_readings(input: &[u8], keep: usize) -> Vec<u32>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `clamp_property_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3)` gives `vec![1u32, 0u32]`.
- `clamp_property_frequent_readings(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1649/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1649`.

[Module reading](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
