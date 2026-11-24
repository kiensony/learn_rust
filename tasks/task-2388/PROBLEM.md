# task-2388 — Rolling parity · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Collect a cumulative trace over a byte stream. At each position observe the bitwise XOR of all bytes so far. Use a single pass with constant extra bookkeeping beyond the result. Add a slow prefix-by-prefix oracle to a test and compare on a stream of at least 100 entries. Explain the number of element visits in each implementation.

Return the absolute differences between each adjacent pair, in order. Use incremental state and an independent performance model. During review, point to the operation that enforces its contract.

```rust
pub fn rolling_parity_steps(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `rolling_parity_steps(&[])` gives `vec![]`.
- `rolling_parity_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2388/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2388`.

[Module reading](https://doc.rust-lang.org/std/time/struct.Instant.html)
