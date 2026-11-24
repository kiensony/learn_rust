# task-970 — Iterate over matching fields · longest field with last tie (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Produce a Vec of one selected byte length per record in input order using an iterator adapter. Keep duplicates and zero lengths.

```rust
pub fn longest_field_with_last_tie_slash<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `longest_field_with_last_tie_slash(vec!["path/1/12", "path/123"])` gives `vec![2usize, 3usize]`.
- `longest_field_with_last_tie_slash(vec!["path/123", "hi/kit"])` gives `vec![3usize, 3usize]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-970/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-970`.

[Module reading](https://doc.rust-lang.org/book/ch13-02-iterators.html)
