# task-998 — Rebuild runs without consecutive duplicates

Consume the iterator once. Return (value, run length) for each maximal run of equal consecutive values. Separate runs of the same value stay separate.

```rust
pub fn runs(values: impl IntoIterator<Item=u8>) -> Vec<(u8,usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `runs(vec![3, 3, 1, 1, 3])` gives `vec![(3, 2), (1, 2), (3, 1)]`.
- `runs(vec![7])` gives `vec![(7, 1)]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-998/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-998`.

[Module reading](https://doc.rust-lang.org/book/ch13-02-iterators.html)
