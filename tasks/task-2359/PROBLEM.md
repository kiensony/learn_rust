# task-2359 — Single pass prefix maximum · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Collect a cumulative trace over a byte stream. At each position observe the maximum value seen so far. Use a single pass with constant extra bookkeeping beyond the result. Add a slow prefix-by-prefix oracle to a test and compare on a stream of at least 100 entries. Explain the number of element visits in each implementation.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use incremental state and an independent performance model. During review, point to the operation that enforces its contract.

```rust
pub fn single_pass_prefix_maximum_longest_run(input: &[u8]) -> u64
```

## Public examples

- `single_pass_prefix_maximum_longest_run(&[])` gives `0`.
- `single_pass_prefix_maximum_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2359/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2359`.

[Module reading](https://doc.rust-lang.org/std/time/struct.Instant.html)
