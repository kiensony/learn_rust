# task-1562 — Feature padding · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Each byte counts units. Observe the byte value multiplied by the active unit width. The default width is one. With the wide feature it is 4. Add a cfg-gated regression for wide and run cargo test --features wide from this package. Test both configurations. Put your own feature-specific case in a new integration test.

Return the sum of all observations, widening each one before adding. Use Cargo features and cfg!. During review, point to the operation that enforces its contract.

```rust
pub fn feature_padding_total(input: &[u8]) -> u64
```

## Public examples

- `feature_padding_total(&[])` gives `0` by default and `0` with `wide`.
- `feature_padding_total(&[4])` gives `4` by default and `16` with `wide`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1562/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1562`.

[Module reading](https://doc.rust-lang.org/cargo/reference/features.html)
