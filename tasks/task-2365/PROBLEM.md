# task-2365 — Single pass prefix minimum · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Collect a cumulative trace over a byte stream. At each position observe the minimum value seen so far. Use a single pass with constant extra bookkeeping beyond the result. Add a slow prefix-by-prefix oracle to a test and compare on a stream of at least 100 entries. Explain the number of element visits in each implementation.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use incremental state and an independent performance model. During review, point to the operation that enforces its contract.

```rust
pub fn single_pass_prefix_minimum_changes(input: &[u8]) -> Vec<u32>
```

## Public examples

- `single_pass_prefix_minimum_changes(&[])` gives `vec![]`.
- `single_pass_prefix_minimum_changes(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2365/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2365`.

[Module reading](https://doc.rust-lang.org/std/time/struct.Instant.html)
