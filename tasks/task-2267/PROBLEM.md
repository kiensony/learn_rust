# task-2267 — Hot code addresses · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Each input byte is a bucket ID recorded by a sampling profiler. Observe bucket IDs sampled at least twice, sorted by ID. During review explain the difference between sample counts and exact call counts. Benchmark runs should exclude formatting and file parsing.

Return distinct observations in ascending order. Use a histogram built from a deterministic profiler trace. During review, point to the operation that enforces its contract.

```rust
pub fn hot_code_addresses_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `hot_code_addresses_distinct(&[])` gives `vec![]`.
- `hot_code_addresses_distinct(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2267/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2267`.

[Module reading](https://doc.rust-lang.org/rustc/instrument-coverage.html)
