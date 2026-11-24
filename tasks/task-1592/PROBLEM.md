# task-1592 — Feature pages · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Each byte counts units. Observe the byte value multiplied by the active unit width. The default width is one. With the wide feature it is 32. Add a cfg-gated regression for wide and run cargo test --features wide from this package. Test both configurations. Put your own feature-specific case in a new integration test.

Return the sum of all observations, widening each one before adding. Use Cargo features and cfg!. During review, point to the operation that enforces its contract.

```rust
pub fn feature_pages_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `feature_pages_total(&[])` gives `0` by default and `0` with `wide`.
- `feature_pages_total(&[4])` gives `4` by default and `128` with `wide`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1592/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1592`.

[Module reading](https://doc.rust-lang.org/cargo/reference/features.html)
