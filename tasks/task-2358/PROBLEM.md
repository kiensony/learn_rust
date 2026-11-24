# task-2358 — Single pass prefix maximum · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Collect a cumulative trace over a byte stream. At each position observe the maximum value seen so far. Use a single pass with constant extra bookkeeping beyond the result. Add a slow prefix-by-prefix oracle to a test and compare on a stream of at least 100 entries. Explain the number of element visits in each implementation.

Return the absolute differences between each adjacent pair, in order. Use incremental state and an independent performance model. During review, point to the operation that enforces its contract.

```rust
pub fn single_pass_prefix_maximum_steps(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `single_pass_prefix_maximum_steps(&[])` gives `vec![]`.
- `single_pass_prefix_maximum_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2358/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2358`.

[Module reading](https://doc.rust-lang.org/std/time/struct.Instant.html)
