# task-2139 — Histogram bins · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

A scheduler receives one u8 key per entry. Observe the frequency per occupied bin in ascending key order. Explain the cost of inserting and extracting the next item.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use the standard library's BTreeMap::entry. During review, point to the operation that enforces its contract.

```rust
pub fn histogram_bins_longest_run(input: &[u8]) -> u64
```

## Public examples

- `histogram_bins_longest_run(&[])` gives `0`.
- `histogram_bins_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2139/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2139`.

[Module reading](https://doc.rust-lang.org/std/collections/)
