# task-2288 — Weighted pressure · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Each input byte is a bucket ID recorded by a sampling profiler. Observe the product of bucket ID and frequency in ascending occupied bucket order. During review explain the difference between sample counts and exact call counts. Benchmark runs should exclude formatting and file parsing.

Return the absolute differences between each adjacent pair, in order. Use a histogram built from a deterministic profiler trace. During review, point to the operation that enforces its contract.

```rust
pub fn weighted_pressure_steps(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `weighted_pressure_steps(&[])` gives `vec![]`.
- `weighted_pressure_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2288/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2288`.

[Module reading](https://doc.rust-lang.org/rustc/instrument-coverage.html)
