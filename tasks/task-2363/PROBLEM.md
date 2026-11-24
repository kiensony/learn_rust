# task-2363 — Single pass prefix minimum · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Collect a cumulative trace over a byte stream. At each position observe the minimum value seen so far. Use a single pass with constant extra bookkeeping beyond the result. Add a slow prefix-by-prefix oracle to a test and compare on a stream of at least 100 entries. Explain the number of element visits in each implementation.

Return Some((minimum, maximum)) or None for an empty trace. Use incremental state and an independent performance model. During review, point to the operation that enforces its contract.

```rust
pub fn single_pass_prefix_minimum_bounds(input: &[u8]) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `single_pass_prefix_minimum_bounds(&[])` gives `None`.
- `single_pass_prefix_minimum_bounds(&[4])` gives `Some((4u32, 4u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2363/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2363`.

[Module reading](https://doc.rust-lang.org/std/time/struct.Instant.html)
