# task-1197 — Build two borrowed columns

Split each valid record at its first colon and keep both slices borrowed from the source. Skip separator-free lines and preserve order, duplicates and empty fields. Allocate only the result vector.

```rust
pub fn columns_of<'a>(records: &'a [&'a str]) -> Vec<(&'a str,&'a str)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `columns_of(&["a:b", "c:d"])` gives `vec![("a","b"),("c","d")]`.
- `columns_of(&["none", "x:y:z"])` gives `vec![("x","y:z")]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1197/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1197`.

[Module reading](https://doc.rust-lang.org/reference/trait-bounds.html)
