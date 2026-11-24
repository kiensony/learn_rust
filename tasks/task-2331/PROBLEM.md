# task-2331 — Batch variation · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

The input is a fixed sequence of benchmark durations in microseconds. Observe the range (largest minus smallest duration) in consecutive nonempty batches of four. Run the acceptance suite with both cargo test and cargo test --release, and explain how compiler optimization could affect a timing-based assertion.

Return the observations in input order, including duplicates. Use deterministic measurement fixtures and a release build. During review, point to the operation that enforces its contract.

```rust
pub fn batch_variation_trace(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `batch_variation_trace(&[])` gives `vec![]`.
- `batch_variation_trace(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2331/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2331`.

[Module reading](https://doc.rust-lang.org/cargo/reference/profiles.html)
