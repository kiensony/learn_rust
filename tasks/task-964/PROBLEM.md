# task-964 — Iterate over matching fields · leading marks removed (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Produce a Vec of one selected byte length per record in input order using an iterator adapter. Keep duplicates and zero lengths.

```rust
pub fn leading_marks_removed_slash<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_marks_removed_slash(vec!["path/1/12", "path/123"])` gives `vec![4usize, 3usize]`.
- `leading_marks_removed_slash(vec!["path/123", "hi/kit"])` gives `vec![3usize, 6usize]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-964/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-964`.

[Module reading](https://doc.rust-lang.org/book/ch13-02-iterators.html)
