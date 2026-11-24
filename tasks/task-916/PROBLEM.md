# task-916 — Call a supplied closure · closing marks removed (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn closing_marks_removed_slash(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_marks_removed_slash("path/1/12", |s| s.chars().count() + 1)` gives `5`.
- `closing_marks_removed_slash("path/123", |s| s.len() * 2)` gives `6`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-916/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-916`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
