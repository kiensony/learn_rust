# task-2343 — Baseline drift · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

The input is a fixed sequence of benchmark durations in microseconds. Observe the absolute difference between each duration and the first measurement. Run the acceptance suite with both cargo test and cargo test --release, and explain how compiler optimization could affect a timing-based assertion.

Return Some((minimum, maximum)) or None for an empty trace. Use deterministic measurement fixtures and a release build. During review, point to the operation that enforces its contract.

```rust
pub fn baseline_drift_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `baseline_drift_bounds(&[])` gives `None`.
- `baseline_drift_bounds(&[4])` gives `Some((0u32, 0u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2343/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2343`.

[Module reading](https://doc.rust-lang.org/cargo/reference/profiles.html)
