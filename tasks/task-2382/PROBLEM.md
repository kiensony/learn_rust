# task-2382 — Rolling parity · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Collect a cumulative trace over a byte stream. At each position observe the bitwise XOR of all bytes so far. Use a single pass with constant extra bookkeeping beyond the result. Add a slow prefix-by-prefix oracle to a test and compare on a stream of at least 100 entries. Explain the number of element visits in each implementation.

Return the sum of all observations, widening each one before adding. Use incremental state and an independent performance model. During review, point to the operation that enforces its contract.

```rust
pub fn rolling_parity_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `rolling_parity_total(&[])` gives `0`.
- `rolling_parity_total(&[4])` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2382/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2382`.

[Module reading](https://doc.rust-lang.org/std/time/struct.Instant.html)
