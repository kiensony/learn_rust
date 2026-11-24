# task-2260 — CPU sample counts · reverse

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Each input byte is a bucket ID recorded by a sampling profiler. Observe the number of samples per occupied program-counter bucket in ascending bucket order. During review explain the difference between sample counts and exact call counts. Benchmark runs should exclude formatting and file parsing.

Return the observations in reverse order. Retain duplicate observations. Use a histogram built from a deterministic profiler trace. During review, point to the operation that enforces its contract.

```rust
pub fn cpu_sample_counts_reverse(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `cpu_sample_counts_reverse(&[])` gives `vec![]`.
- `cpu_sample_counts_reverse(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2260/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2260`.

[Module reading](https://doc.rust-lang.org/rustc/instrument-coverage.html)
