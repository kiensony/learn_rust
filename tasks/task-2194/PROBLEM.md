# task-2194 — Duplicate insertion cost · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Instrument an algorithm on a stream of keys. Observe the count of distinct keys stored after each insertion. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Keep the first observation and every later one strictly above all earlier observations. Use deterministic operation counts independent of wall-clock time. During review, point to the operation that enforces its contract.

```rust
pub fn duplicate_insertion_cost_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `duplicate_insertion_cost_peaks(&[])` gives `vec![]`.
- `duplicate_insertion_cost_peaks(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2194/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2194`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
