# task-2107 — Sorted keys · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

A scheduler receives one u8 key per entry. Observe the distinct keys in sorted order. Explain the cost of inserting and extracting the next item.

Return distinct observations in ascending order. Use the standard library's BTreeSet. During review, point to the operation that enforces its contract.

```rust
pub fn sorted_keys_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `sorted_keys_distinct(&[])` gives `vec![]`.
- `sorted_keys_distinct(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2107/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2107`.

[Module reading](https://doc.rust-lang.org/std/collections/)
