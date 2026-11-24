# task-1607 — Parity property · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Each byte is a generated test input. Observe one when the following predicate holds and zero otherwise: the remainder on division by two differs from the low bit. As a separate regression test, enumerate the full u8 domain; check the property against an independently computed model and report the smallest counterexample.

Return distinct observations in ascending order. Use an exhaustive u8 domain test with a model separate from the implementation. During review, point to the operation that enforces its contract.

```rust
pub fn parity_property_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `parity_property_distinct(&[])` gives `vec![]`.
- `parity_property_distinct(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1607/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1607`.

[Module reading](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
