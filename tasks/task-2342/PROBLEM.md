# task-2342 — Baseline drift · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

The input is a fixed sequence of benchmark durations in microseconds. Observe the absolute difference between each duration and the first measurement. Run the acceptance suite with both cargo test and cargo test --release, and explain how compiler optimization could affect a timing-based assertion.

Return the sum of all observations, widening each one before adding. Use deterministic measurement fixtures and a release build. During review, point to the operation that enforces its contract.

```rust
pub fn baseline_drift_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `baseline_drift_total(&[])` gives `0`.
- `baseline_drift_total(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2342/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2342`.

[Module reading](https://doc.rust-lang.org/cargo/reference/profiles.html)
