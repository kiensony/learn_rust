# task-2252 — CPU sample counts · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Each input byte is a bucket ID recorded by a sampling profiler. Observe the number of samples per occupied program-counter bucket in ascending bucket order. During review explain the difference between sample counts and exact call counts. Benchmark runs should exclude formatting and file parsing.

Return the sum of all observations, widening each one before adding. Use a histogram built from a deterministic profiler trace. During review, point to the operation that enforces its contract.

```rust
pub fn cpu_sample_counts_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `cpu_sample_counts_total(&[])` gives `0`.
- `cpu_sample_counts_total(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2252/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2252`.

[Module reading](https://doc.rust-lang.org/rustc/instrument-coverage.html)
