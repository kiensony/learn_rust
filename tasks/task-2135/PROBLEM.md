# task-2135 — Histogram bins · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

A scheduler receives one u8 key per entry. Observe the frequency per occupied bin in ascending key order. Explain the cost of inserting and extracting the next item.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use the standard library's BTreeMap::entry. During review, point to the operation that enforces its contract.

```rust
pub fn histogram_bins_changes(input: &[u8]) -> Vec<u32>
```

## Public examples

- `histogram_bins_changes(&[])` gives `vec![]`.
- `histogram_bins_changes(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2135/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2135`.

[Module reading](https://doc.rust-lang.org/std/collections/)
