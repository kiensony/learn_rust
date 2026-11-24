# task-2341 — Baseline drift · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

The input is a fixed sequence of benchmark durations in microseconds. Observe the absolute difference between each duration and the first measurement. Run the acceptance suite with both cargo test and cargo test --release, and explain how compiler optimization could affect a timing-based assertion.

Return the observations in input order, including duplicates. Use deterministic measurement fixtures and a release build. During review, point to the operation that enforces its contract.

```rust
pub fn baseline_drift_trace(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `baseline_drift_trace(&[])` gives `vec![]`.
- `baseline_drift_trace(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2341/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2341`.

[Module reading](https://doc.rust-lang.org/cargo/reference/profiles.html)
