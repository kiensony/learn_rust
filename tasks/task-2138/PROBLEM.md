# task-2138 — Histogram bins · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

A scheduler receives one u8 key per entry. Observe the frequency per occupied bin in ascending key order. Explain the cost of inserting and extracting the next item.

Return the absolute differences between each adjacent pair, in order. Use the standard library's BTreeMap::entry. During review, point to the operation that enforces its contract.

```rust
pub fn histogram_bins_steps(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `histogram_bins_steps(&[])` gives `vec![]`.
- `histogram_bins_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2138/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2138`.

[Module reading](https://doc.rust-lang.org/std/collections/)
