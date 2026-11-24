# task-2277 — Exclusive hotspots · distinct

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Each input byte is a bucket ID recorded by a sampling profiler. Observe IDs sampled exactly once, sorted by ID. During review explain the difference between sample counts and exact call counts. Benchmark runs should exclude formatting and file parsing.

Return distinct observations in ascending order. Use a histogram built from a deterministic profiler trace. During review, point to the operation that enforces its contract.

```rust
pub fn exclusive_hotspots_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `exclusive_hotspots_distinct(&[])` gives `vec![]`.
- `exclusive_hotspots_distinct(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2277/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2277`.

[Module reading](https://doc.rust-lang.org/rustc/instrument-coverage.html)
