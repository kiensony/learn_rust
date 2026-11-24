# task-817 — Report field errors · marks removed at both ends (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length. Use FieldError::Missing for an empty field and FieldError::TooLong above 4 bytes.

```rust
pub fn marks_removed_at_both_ends_slash(text: &str) -> Result<usize, FieldError>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `marks_removed_at_both_ends_slash("path/1/12")` gives `Ok(4)`.
- `marks_removed_at_both_ends_slash("path/123")` gives `Ok(3)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-817/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-817`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
