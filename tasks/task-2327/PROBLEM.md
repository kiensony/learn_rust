# task-2327 — Batch medians · distinct

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

The input is a fixed sequence of benchmark durations in microseconds. Observe the lower median in each consecutive group of up to three measurements. Run the acceptance suite with both cargo test and cargo test --release, and explain how compiler optimization could affect a timing-based assertion.

Return distinct observations in ascending order. Use deterministic measurement fixtures and a release build. During review, point to the operation that enforces its contract.

```rust
pub fn batch_medians_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `batch_medians_distinct(&[])` gives `vec![]`.
- `batch_medians_distinct(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2327/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2327`.

[Module reading](https://doc.rust-lang.org/cargo/reference/profiles.html)
