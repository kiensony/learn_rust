# task-2283 — Weighted pressure · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Each input byte is a bucket ID recorded by a sampling profiler. Observe the product of bucket ID and frequency in ascending occupied bucket order. During review explain the difference between sample counts and exact call counts. Benchmark runs should exclude formatting and file parsing.

Return Some((minimum, maximum)) or None for an empty trace. Use a histogram built from a deterministic profiler trace. During review, point to the operation that enforces its contract.

```rust
pub fn weighted_pressure_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `weighted_pressure_bounds(&[])` gives `None`.
- `weighted_pressure_bounds(&[4])` gives `Some((4u32, 4u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2283/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2283`.

[Module reading](https://doc.rust-lang.org/rustc/instrument-coverage.html)
