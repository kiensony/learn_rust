# task-846 — Report field errors · closing marks removed (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length. Use FieldError::Missing for an empty field and FieldError::TooLong above 7 bytes.

```rust
pub fn closing_marks_removed_hash(text: &str) -> Result<usize, FieldError>
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `closing_marks_removed_hash("outside[red#blue]ignored")` gives `Err(FieldError::TooLong)`.
- `closing_marks_removed_hash("[猫###a]")` gives `Ok(7)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-846/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-846`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
