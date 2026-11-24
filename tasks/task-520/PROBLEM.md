# task-520 — Measure Unicode text · leading marks removed (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. After selecting the field, count its Unicode scalar values. An emoji occupies one scalar value in the examples.

```rust
pub fn leading_marks_removed_slash(text: &str) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_marks_removed_slash("path/1/12")` gives `4`.
- `leading_marks_removed_slash("path/123")` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-520/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-520`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
