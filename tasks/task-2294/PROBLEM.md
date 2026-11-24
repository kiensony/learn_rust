# task-2294 — Profile ranking · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Each input byte is a bucket ID recorded by a sampling profiler. Observe bucket IDs in descending sample-count order, breaking ties with the larger ID first. During review explain the difference between sample counts and exact call counts. Benchmark runs should exclude formatting and file parsing.

Keep the first observation and every later one strictly above all earlier observations. Use a histogram built from a deterministic profiler trace. During review, point to the operation that enforces its contract.

```rust
pub fn profile_ranking_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `profile_ranking_peaks(&[])` gives `vec![]`.
- `profile_ranking_peaks(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2294/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2294`.

[Module reading](https://doc.rust-lang.org/rustc/instrument-coverage.html)
