# task-2268 — Hot code addresses · steps

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Each input byte is a bucket ID recorded by a sampling profiler. Observe bucket IDs sampled at least twice, sorted by ID. During review explain the difference between sample counts and exact call counts. Benchmark runs should exclude formatting and file parsing.

Return the absolute differences between each adjacent pair, in order. Use a histogram built from a deterministic profiler trace. During review, point to the operation that enforces its contract.

```rust
pub fn hot_code_addresses_steps(input: &[u8]) -> Vec<u32>
```

## Public examples

- `hot_code_addresses_steps(&[])` gives `vec![]`.
- `hot_code_addresses_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2268/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2268`.

[Module reading](https://doc.rust-lang.org/rustc/instrument-coverage.html)
