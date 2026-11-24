# task-2390 — Rolling parity · reverse

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Collect a cumulative trace over a byte stream. At each position observe the bitwise XOR of all bytes so far. Use a single pass with constant extra bookkeeping beyond the result. Add a slow prefix-by-prefix oracle to a test and compare on a stream of at least 100 entries. Explain the number of element visits in each implementation.

Return the observations in reverse order. Retain duplicate observations. Use incremental state and an independent performance model. During review, point to the operation that enforces its contract.

```rust
pub fn rolling_parity_reverse(input: &[u8]) -> Vec<u32>
```

## Public examples

- `rolling_parity_reverse(&[])` gives `vec![]`.
- `rolling_parity_reverse(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2390/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2390`.

[Module reading](https://doc.rust-lang.org/std/time/struct.Instant.html)
