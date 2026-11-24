# task-2279 — Exclusive hotspots · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Each input byte is a bucket ID recorded by a sampling profiler. Observe IDs sampled exactly once, sorted by ID. During review explain the difference between sample counts and exact call counts. Benchmark runs should exclude formatting and file parsing.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use a histogram built from a deterministic profiler trace. During review, point to the operation that enforces its contract.

```rust
pub fn exclusive_hotspots_longest_run(input: &[u8]) -> u64
```

## Public examples

- `exclusive_hotspots_longest_run(&[])` gives `0`.
- `exclusive_hotspots_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2279/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2279`.

[Module reading](https://doc.rust-lang.org/rustc/instrument-coverage.html)
