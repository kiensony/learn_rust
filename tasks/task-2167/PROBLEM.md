# task-2167 — Sorted insertion moves · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Instrument an algorithm on a stream of keys. Observe the number of older elements shifted right when each key is inserted stably after any equals in a sorted vector. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Return distinct observations in ascending order. Use deterministic operation counts independent of wall-clock time. During review, point to the operation that enforces its contract.

```rust
pub fn sorted_insertion_moves_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `sorted_insertion_moves_distinct(&[])` gives `vec![]`.
- `sorted_insertion_moves_distinct(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2167/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2167`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
