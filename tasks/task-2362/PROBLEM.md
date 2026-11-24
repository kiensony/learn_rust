# task-2362 — Single pass prefix minimum · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Collect a cumulative trace over a byte stream. At each position observe the minimum value seen so far. Use a single pass with constant extra bookkeeping beyond the result. Add a slow prefix-by-prefix oracle to a test and compare on a stream of at least 100 entries. Explain the number of element visits in each implementation.

Return the sum of all observations, widening each one before adding. Use incremental state and an independent performance model. During review, point to the operation that enforces its contract.

```rust
pub fn single_pass_prefix_minimum_total(input: &[u8]) -> u64
```

## Public examples

- `single_pass_prefix_minimum_total(&[])` gives `0`.
- `single_pass_prefix_minimum_total(&[4])` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2362/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2362`.

[Module reading](https://doc.rust-lang.org/std/time/struct.Instant.html)
