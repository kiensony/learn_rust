# task-824 — Report field errors · closing field (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length. Use FieldError::Missing for an empty field and FieldError::TooLong above 5 bytes.

```rust
pub fn closing_field_pipe(text: &str) -> Result<usize, FieldError>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_field_pipe("first\n123")` gives `Ok(3)`.
- `closing_field_pipe("|é|a")` gives `Ok(1)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-824/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-824`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
