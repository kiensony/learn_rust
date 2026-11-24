# task-2289 — Weighted pressure · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Each input byte is a bucket ID recorded by a sampling profiler. Observe the product of bucket ID and frequency in ascending occupied bucket order. During review explain the difference between sample counts and exact call counts. Benchmark runs should exclude formatting and file parsing.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use a histogram built from a deterministic profiler trace. During review, point to the operation that enforces its contract.

```rust
pub fn weighted_pressure_longest_run(input: &[u8]) -> u64
```

## Public examples

- `weighted_pressure_longest_run(&[])` gives `0`.
- `weighted_pressure_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2289/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2289`.

[Module reading](https://doc.rust-lang.org/rustc/instrument-coverage.html)
