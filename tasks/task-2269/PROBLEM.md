# task-2269 — Hot code addresses · longest run

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Each input byte is a bucket ID recorded by a sampling profiler. Observe bucket IDs sampled at least twice, sorted by ID. During review explain the difference between sample counts and exact call counts. Benchmark runs should exclude formatting and file parsing.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use a histogram built from a deterministic profiler trace. During review, point to the operation that enforces its contract.

```rust
pub fn hot_code_addresses_longest_run(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `hot_code_addresses_longest_run(&[])` gives `0`.
- `hot_code_addresses_longest_run(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2269/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2269`.

[Module reading](https://doc.rust-lang.org/rustc/instrument-coverage.html)
