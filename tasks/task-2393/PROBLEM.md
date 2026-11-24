# task-2393 — Last change age · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Collect a cumulative trace over a byte stream. At each position observe the length of the current run of consecutive equal bytes. Use a single pass with constant extra bookkeeping beyond the result. Add a slow prefix-by-prefix oracle to a test and compare on a stream of at least 100 entries. Explain the number of element visits in each implementation.

Return Some((minimum, maximum)) or None for an empty trace. Use incremental state and an independent performance model. During review, point to the operation that enforces its contract.

```rust
pub fn last_change_age_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `last_change_age_bounds(&[])` gives `None`.
- `last_change_age_bounds(&[4])` gives `Some((1u32, 1u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2393/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2393`.

[Module reading](https://doc.rust-lang.org/std/time/struct.Instant.html)
