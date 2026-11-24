# task-2137 — Histogram bins · distinct

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

A scheduler receives one u8 key per entry. Observe the frequency per occupied bin in ascending key order. Explain the cost of inserting and extracting the next item.

Return distinct observations in ascending order. Use the standard library's BTreeMap::entry. During review, point to the operation that enforces its contract.

```rust
pub fn histogram_bins_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `histogram_bins_distinct(&[])` gives `vec![]`.
- `histogram_bins_distinct(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2137/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2137`.

[Module reading](https://doc.rust-lang.org/std/collections/)
