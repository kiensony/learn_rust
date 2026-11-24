# task-2257 — CPU sample counts · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Each input byte is a bucket ID recorded by a sampling profiler. Observe the number of samples per occupied program-counter bucket in ascending bucket order. During review explain the difference between sample counts and exact call counts. Benchmark runs should exclude formatting and file parsing.

Return distinct observations in ascending order. Use a histogram built from a deterministic profiler trace. During review, point to the operation that enforces its contract.

```rust
pub fn cpu_sample_counts_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `cpu_sample_counts_distinct(&[])` gives `vec![]`.
- `cpu_sample_counts_distinct(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2257/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2257`.

[Module reading](https://doc.rust-lang.org/rustc/instrument-coverage.html)
