# task-1556 — Feature scaling · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Each byte counts units. Observe the byte value multiplied by the active unit width. The default width is one. With the wide feature it is 2. Add a cfg-gated regression for wide and run cargo test --features wide from this package. Test both configurations. Put your own feature-specific case in a new integration test.

Count adjacent pairs for which the second observation is strictly smaller. Use Cargo features and cfg!. During review, point to the operation that enforces its contract.

```rust
pub fn feature_scaling_descending_steps(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `feature_scaling_descending_steps(&[])` gives `0` by default and `0` with `wide`.
- `feature_scaling_descending_steps(&[4])` gives `0` by default and `0` with `wide`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1556/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1556`.

[Module reading](https://doc.rust-lang.org/cargo/reference/features.html)
