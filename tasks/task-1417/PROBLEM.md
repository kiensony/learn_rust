# task-1417 — Elapsed seconds · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the whole elapsed seconds.

Return distinct observations in ascending order. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn elapsed_seconds_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `elapsed_seconds_distinct(&[])` gives `vec![]`.
- `elapsed_seconds_distinct(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1417/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1417`.

[Module reading](https://doc.rust-lang.org/std/time/)
