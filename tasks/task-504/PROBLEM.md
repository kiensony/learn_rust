# task-504 — Unzip measurement pairs

Split pairs into two columns. Keep the original row order and preserve repeated values in each column.

```rust
pub fn columns(rows: &[(i8, i8)]) -> (Vec<i8>, Vec<i8>)
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `columns(&[(2, 3), (4, 5)])` gives `(vec![2, 4], vec![3, 5])`.
- `columns(&[(1, -1)])` gives `(vec![1], vec![-1])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-504/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-504`.

[Module reading](https://doc.rust-lang.org/book/ch04-03-slices.html)
