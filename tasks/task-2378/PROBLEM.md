# task-2378 — Cumulative occupancy · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Collect a cumulative trace over a byte stream. At each position observe the number of nonzero entries seen so far. Use a single pass with constant extra bookkeeping beyond the result. Add a slow prefix-by-prefix oracle to a test and compare on a stream of at least 100 entries. Explain the number of element visits in each implementation.

Return the absolute differences between each adjacent pair, in order. Use incremental state and an independent performance model. During review, point to the operation that enforces its contract.

```rust
pub fn cumulative_occupancy_steps(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `cumulative_occupancy_steps(&[])` gives `vec![]`.
- `cumulative_occupancy_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2378/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2378`.

[Module reading](https://doc.rust-lang.org/std/time/struct.Instant.html)
