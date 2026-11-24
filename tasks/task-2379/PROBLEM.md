# task-2379 — Cumulative occupancy · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Collect a cumulative trace over a byte stream. At each position observe the number of nonzero entries seen so far. Use a single pass with constant extra bookkeeping beyond the result. Add a slow prefix-by-prefix oracle to a test and compare on a stream of at least 100 entries. Explain the number of element visits in each implementation.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use incremental state and an independent performance model. During review, point to the operation that enforces its contract.

```rust
pub fn cumulative_occupancy_longest_run(input: &[u8]) -> u64
```

## Public examples

- `cumulative_occupancy_longest_run(&[])` gives `0`.
- `cumulative_occupancy_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2379/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2379`.

[Module reading](https://doc.rust-lang.org/std/time/struct.Instant.html)
