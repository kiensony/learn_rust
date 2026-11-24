# task-969 — Iterate over matching fields · first nonempty field (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Produce a Vec of one selected byte length per record in input order using an iterator adapter. Keep duplicates and zero lengths.

```rust
pub fn first_nonempty_field_slash<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `first_nonempty_field_slash(vec!["path/1/12", "path/123"])` gives `vec![1usize, 3usize]`.
- `first_nonempty_field_slash(vec!["path/123", "hi/kit"])` gives `vec![3usize, 2usize]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-969/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-969`.

[Module reading](https://doc.rust-lang.org/book/ch13-02-iterators.html)
