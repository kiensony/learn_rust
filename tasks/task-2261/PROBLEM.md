# task-2261 — Hot code addresses · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Each input byte is a bucket ID recorded by a sampling profiler. Observe bucket IDs sampled at least twice, sorted by ID. During review explain the difference between sample counts and exact call counts. Benchmark runs should exclude formatting and file parsing.

Return the observations in input order, including duplicates. Use a histogram built from a deterministic profiler trace. During review, point to the operation that enforces its contract.

```rust
pub fn hot_code_addresses_trace(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `hot_code_addresses_trace(&[])` gives `vec![]`.
- `hot_code_addresses_trace(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2261/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2261`.

[Module reading](https://doc.rust-lang.org/rustc/instrument-coverage.html)
