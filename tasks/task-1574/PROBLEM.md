# task-1574 — Feature packing · peaks

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Each byte counts units. Observe the byte value multiplied by the active unit width. The default width is one. With the wide feature it is 8. Add a cfg-gated regression for wide and run cargo test --features wide from this package. Test both configurations. Put your own feature-specific case in a new integration test.

Keep the first observation and every later one strictly above all earlier observations. Use Cargo features and cfg!. During review, point to the operation that enforces its contract.

```rust
pub fn feature_packing_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `feature_packing_peaks(&[])` gives `vec![]` by default and `vec![]` with `wide`.
- `feature_packing_peaks(&[4])` gives `vec![4u32]` by default and `vec![32u32]` with `wide`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1574/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1574`.

[Module reading](https://doc.rust-lang.org/cargo/reference/features.html)
