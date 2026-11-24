# task-954 — Iterate over matching fields · closing field (:)

Treat ':' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Produce a Vec of one selected byte length per record in input order using an iterator adapter. Keep duplicates and zero lengths.

```rust
pub fn closing_field_colon<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_field_colon(vec!["hi:kit", ":é:a"])` gives `vec![3usize, 1usize]`.
- `closing_field_colon(vec![":é:a", ""])` gives `vec![1usize, 0usize]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-954/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-954`.

[Module reading](https://doc.rust-lang.org/book/ch13-02-iterators.html)
