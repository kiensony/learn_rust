# task-1598 — Feature packing · rolling load

Each byte counts units. Observe the byte value multiplied by the active unit width. The default width is one. With the wide feature it is 8. Add a cfg-gated regression for wide and run cargo test --features wide from this package. Test both configurations. Put your own feature-specific case in a new integration test.

For each complete sliding window of width observations, return its sum as u64. Consecutive windows overlap by width minus one observations. Width zero and widths greater than the trace length yield an empty vector.

Use Cargo features and cfg!. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn feature_packing_rolling_load(input: &[u8], width: usize) -> Vec<u64>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `feature_packing_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1)` gives `vec![3u64, 6u64, 9u64, 12u64, 21u64, 15u64, 0u64, 26u64, 8u64, 31u64]` by default and `vec![24u64, 48u64, 72u64, 96u64, 168u64, 120u64, 0u64, 208u64, 64u64, 248u64]` with `wide`.
- `feature_packing_rolling_load(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1598/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1598`.

[Module reading](https://doc.rust-lang.org/cargo/reference/features.html)
