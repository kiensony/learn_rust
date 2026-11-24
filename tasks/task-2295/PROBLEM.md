# task-2295 — Profile ranking · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Each input byte is a bucket ID recorded by a sampling profiler. Observe bucket IDs in descending sample-count order, breaking ties with the larger ID first. During review explain the difference between sample counts and exact call counts. Benchmark runs should exclude formatting and file parsing.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use a histogram built from a deterministic profiler trace. During review, point to the operation that enforces its contract.

```rust
pub fn profile_ranking_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `profile_ranking_changes(&[])` gives `vec![]`.
- `profile_ranking_changes(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2295/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2295`.

[Module reading](https://doc.rust-lang.org/rustc/instrument-coverage.html)
