# task-1097 — Measure mixed shapes through a trait

Compute the sum of spans through the Span trait for a slice of any one implementing type. Support types introduced by the caller. Convert each span to u64 before summing.

```rust
pub fn total_span<T: Span>(items: &[T]) -> u64
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `total_span(&[Length(2), Length(5)])` gives `7`.
- `total_span(&[Interval(4,9), Interval(9,4)])` gives `10`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1097/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1097`.

[Module reading](https://doc.rust-lang.org/reference/items/traits.html)
