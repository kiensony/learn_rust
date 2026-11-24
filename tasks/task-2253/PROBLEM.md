# task-2253 — CPU sample counts · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Each input byte is a bucket ID recorded by a sampling profiler. Observe the number of samples per occupied program-counter bucket in ascending bucket order. During review explain the difference between sample counts and exact call counts. Benchmark runs should exclude formatting and file parsing.

Return Some((minimum, maximum)) or None for an empty trace. Use a histogram built from a deterministic profiler trace. During review, point to the operation that enforces its contract.

```rust
pub fn cpu_sample_counts_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `cpu_sample_counts_bounds(&[])` gives `None`.
- `cpu_sample_counts_bounds(&[4])` gives `Some((1u32, 1u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2253/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2253`.

[Module reading](https://doc.rust-lang.org/rustc/instrument-coverage.html)
