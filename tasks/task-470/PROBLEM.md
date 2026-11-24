# task-470 — Split a slice · leading marks removed (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Inspect the longer slice in the pair (first on a tie). Return the selected field’s byte length.

```rust
pub fn leading_marks_removed_slash(fields: (&str, &str)) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_marks_removed_slash(("path/1/12", "x"))` gives `4`.
- `leading_marks_removed_slash(("path/123", "z/w"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-470/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-470`.

[Module reading](https://doc.rust-lang.org/book/ch04-03-slices.html)
