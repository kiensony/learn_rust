# task-2392 — Last change age · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Collect a cumulative trace over a byte stream. At each position observe the length of the current run of consecutive equal bytes. Use a single pass with constant extra bookkeeping beyond the result. Add a slow prefix-by-prefix oracle to a test and compare on a stream of at least 100 entries. Explain the number of element visits in each implementation.

Return the sum of all observations, widening each one before adding. Use incremental state and an independent performance model. During review, point to the operation that enforces its contract.

```rust
pub fn last_change_age_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `last_change_age_total(&[])` gives `0`.
- `last_change_age_total(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2392/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2392`.

[Module reading](https://doc.rust-lang.org/std/time/struct.Instant.html)
