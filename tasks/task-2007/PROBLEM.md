# task-2007 — Polled lengths · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Create one ready future per byte and poll each exactly once. Observe the ready payload, the byte itself.

Return distinct observations in ascending order. Use a pinned future and an explicit polling context. During review, point to the operation that enforces its contract.

```rust
pub fn polled_lengths_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `polled_lengths_distinct(&[])` gives `vec![]`.
- `polled_lengths_distinct(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2007/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2007`.

[Module reading](https://doc.rust-lang.org/std/future/)
