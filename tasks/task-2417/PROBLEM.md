# task-2417 — Word alignment · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Each byte denotes a logical offset, unrelated to an actual allocation address. Compute the smallest offset at or above it that meets a slot type's alignment. Define the slot with repr(align(4)), query its alignment, and widen before rounding. Observe each aligned offset.

Return distinct observations in ascending order. Use repr(align) and std::mem::align_of. During review, point to the operation that enforces its contract.

```rust
pub fn word_alignment_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `word_alignment_distinct(&[])` gives `vec![]`.
- `word_alignment_distinct(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2417/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2417`.

[Module reading](https://doc.rust-lang.org/reference/type-layout.html)
