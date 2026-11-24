# task-963 — Iterate over matching fields · closing field (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Produce a Vec of one selected byte length per record in input order using an iterator adapter. Keep duplicates and zero lengths.

```rust
pub fn closing_field_slash<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `closing_field_slash(vec!["path/1/12", "path/123"])` gives `vec![2usize, 3usize]`.
- `closing_field_slash(vec!["path/123", "hi/kit"])` gives `vec![3usize, 3usize]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-963/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-963`.

[Module reading](https://doc.rust-lang.org/book/ch13-02-iterators.html)
