# task-2272 — Exclusive hotspots · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Each input byte is a bucket ID recorded by a sampling profiler. Observe IDs sampled exactly once, sorted by ID. During review explain the difference between sample counts and exact call counts. Benchmark runs should exclude formatting and file parsing.

Return the sum of all observations, widening each one before adding. Use a histogram built from a deterministic profiler trace. During review, point to the operation that enforces its contract.

```rust
pub fn exclusive_hotspots_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `exclusive_hotspots_total(&[])` gives `0`.
- `exclusive_hotspots_total(&[4])` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2272/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2272`.

[Module reading](https://doc.rust-lang.org/rustc/instrument-coverage.html)
