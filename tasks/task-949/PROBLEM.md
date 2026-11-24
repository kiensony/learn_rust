# task-949 — Remember the previous sample

Use a closure with a captured previous reading. Return each reading strictly greater than its immediate predecessor. Preserve input order. The first reading has no predecessor.

```rust
pub fn rises(values: &[i16]) -> Vec<i16>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `rises(&[1, 3, 2, 4])` gives `vec![3, 4]`.
- `rises(&[4, 4, 3])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-949/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-949`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
