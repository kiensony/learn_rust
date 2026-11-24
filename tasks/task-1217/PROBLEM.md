# task-1217 — Read chunks · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Read a cursor in blocks of three bytes. Observe the actual number of bytes in every successful nonempty read. Keep a final short block.

Return distinct observations in ascending order. Use Read::read and a fixed buffer. During review, point to the operation that enforces its contract.

```rust
pub fn read_chunks_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `read_chunks_distinct(&[])` gives `vec![]`.
- `read_chunks_distinct(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1217/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1217`.

[Module reading](https://doc.rust-lang.org/std/io/)
