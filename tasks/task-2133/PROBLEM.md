# task-2133 — Histogram bins · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

A scheduler receives one u8 key per entry. Observe the frequency per occupied bin in ascending key order. Explain the cost of inserting and extracting the next item.

Return Some((minimum, maximum)) or None for an empty trace. Use the standard library's BTreeMap::entry. During review, point to the operation that enforces its contract.

```rust
pub fn histogram_bins_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `histogram_bins_bounds(&[])` gives `None`.
- `histogram_bins_bounds(&[4])` gives `Some((1u32, 1u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2133/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2133`.

[Module reading](https://doc.rust-lang.org/std/collections/)
