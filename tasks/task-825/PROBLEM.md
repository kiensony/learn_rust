# task-825 — Report field errors · leading marks removed (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length. Use FieldError::Missing for an empty field and FieldError::TooLong above 5 bytes.

```rust
pub fn leading_marks_removed_pipe(text: &str) -> Result<usize, FieldError>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_marks_removed_pipe("first\n123")` gives `Ok(3)`.
- `leading_marks_removed_pipe("header\n猫|123")` gives `Err(FieldError::TooLong)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-825/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-825`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
