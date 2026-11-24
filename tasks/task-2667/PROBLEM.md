# task-2667 — Lending first byte · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Process consecutive borrowed chunks of up to 3 bytes. For each nonempty chunk observe the first byte. Do not allocate a copy of the chunk. During review connect the lifetime of each view with the API that produced it.

Return distinct observations in ascending order. Use a generic associated type tied to the receiver borrow. During review, point to the operation that enforces its contract.

```rust
pub fn lending_first_byte_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `lending_first_byte_distinct(&[])` gives `vec![]`.
- `lending_first_byte_distinct(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2667/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2667`.

[Module reading](https://doc.rust-lang.org/reference/items/associated-items.html)
