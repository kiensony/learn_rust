# task-2340 — Batch variation · reverse

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

The input is a fixed sequence of benchmark durations in microseconds. Observe the range (largest minus smallest duration) in consecutive nonempty batches of four. Run the acceptance suite with both cargo test and cargo test --release, and explain how compiler optimization could affect a timing-based assertion.

Return the observations in reverse order. Retain duplicate observations. Use deterministic measurement fixtures and a release build. During review, point to the operation that enforces its contract.

```rust
pub fn batch_variation_reverse(input: &[u8]) -> Vec<u32>
```

## Public examples

- `batch_variation_reverse(&[])` gives `vec![]`.
- `batch_variation_reverse(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2340/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2340`.

[Module reading](https://doc.rust-lang.org/cargo/reference/profiles.html)
