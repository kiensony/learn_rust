# task-2132 — Histogram bins · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

A scheduler receives one u8 key per entry. Observe the frequency per occupied bin in ascending key order. Explain the cost of inserting and extracting the next item.

Return the sum of all observations, widening each one before adding. Use the standard library's BTreeMap::entry. During review, point to the operation that enforces its contract.

```rust
pub fn histogram_bins_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `histogram_bins_total(&[])` gives `0`.
- `histogram_bins_total(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2132/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2132`.

[Module reading](https://doc.rust-lang.org/std/collections/)
