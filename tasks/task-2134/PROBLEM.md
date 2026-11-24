# task-2134 — Histogram bins · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

A scheduler receives one u8 key per entry. Observe the frequency per occupied bin in ascending key order. Explain the cost of inserting and extracting the next item.

Keep the first observation and every later one strictly above all earlier observations. Use the standard library's BTreeMap::entry. During review, point to the operation that enforces its contract.

```rust
pub fn histogram_bins_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `histogram_bins_peaks(&[])` gives `vec![]`.
- `histogram_bins_peaks(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2134/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2134`.

[Module reading](https://doc.rust-lang.org/std/collections/)
