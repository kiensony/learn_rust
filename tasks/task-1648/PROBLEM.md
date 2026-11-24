# task-1648 — Round trip property · rolling load

Each byte is a generated test input. Observe one when the following predicate holds and zero otherwise: doubling with wrapping then halving differs from the original. As a separate regression test, enumerate the full u8 domain; check the property against an independently computed model and report the smallest counterexample.

For each complete sliding window of width observations, return its sum as u64. Consecutive windows overlap by width minus one observations. Width zero and widths greater than the trace length yield an empty vector.

Use an exhaustive u8 domain test with a model separate from the implementation. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn round_trip_property_rolling_load(input: &[u8], width: usize) -> Vec<u64>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `round_trip_property_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1)` gives `vec![0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64]`.
- `round_trip_property_rolling_load(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1648/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1648`.

[Module reading](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
