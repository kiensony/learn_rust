# task-2367 — Single pass prefix minimum · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Collect a cumulative trace over a byte stream. At each position observe the minimum value seen so far. Use a single pass with constant extra bookkeeping beyond the result. Add a slow prefix-by-prefix oracle to a test and compare on a stream of at least 100 entries. Explain the number of element visits in each implementation.

Return distinct observations in ascending order. Use incremental state and an independent performance model. During review, point to the operation that enforces its contract.

```rust
pub fn single_pass_prefix_minimum_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `single_pass_prefix_minimum_distinct(&[])` gives `vec![]`.
- `single_pass_prefix_minimum_distinct(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2367/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2367`.

[Module reading](https://doc.rust-lang.org/std/time/struct.Instant.html)
