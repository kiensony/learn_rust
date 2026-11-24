# task-2291 — Profile ranking · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Each input byte is a bucket ID recorded by a sampling profiler. Observe bucket IDs in descending sample-count order, breaking ties with the larger ID first. During review explain the difference between sample counts and exact call counts. Benchmark runs should exclude formatting and file parsing.

Return the observations in input order, including duplicates. Use a histogram built from a deterministic profiler trace. During review, point to the operation that enforces its contract.

```rust
pub fn profile_ranking_trace(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `profile_ranking_trace(&[])` gives `vec![]`.
- `profile_ranking_trace(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2291/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2291`.

[Module reading](https://doc.rust-lang.org/rustc/instrument-coverage.html)
