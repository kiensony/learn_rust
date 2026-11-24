# task-2117 — Priority removal · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

A scheduler receives one u8 key per entry. Observe the priority queue's removal order, largest key first, including duplicates. Explain the cost of inserting and extracting the next item.

Return distinct observations in ascending order. Use the standard library's BinaryHeap. During review, point to the operation that enforces its contract.

```rust
pub fn priority_removal_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `priority_removal_distinct(&[])` gives `vec![]`.
- `priority_removal_distinct(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2117/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2117`.

[Module reading](https://doc.rust-lang.org/std/collections/)
