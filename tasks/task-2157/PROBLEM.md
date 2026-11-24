# task-2157 — Linear search probes · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Instrument an algorithm on a stream of keys. Observe the count of equality probes required by a forward linear search for each input key, including the successful probe. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Return distinct observations in ascending order. Use deterministic operation counts independent of wall-clock time. During review, point to the operation that enforces its contract.

```rust
pub fn linear_search_probes_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `linear_search_probes_distinct(&[])` gives `vec![]`.
- `linear_search_probes_distinct(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2157/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2157`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
