# task-816 — Report field errors · closing marks removed (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length. Use FieldError::Missing for an empty field and FieldError::TooLong above 4 bytes.

```rust
pub fn closing_marks_removed_slash(text: &str) -> Result<usize, FieldError>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_marks_removed_slash("path/1/12")` gives `Ok(4)`.
- `closing_marks_removed_slash("path/123")` gives `Ok(3)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-816/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-816`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
