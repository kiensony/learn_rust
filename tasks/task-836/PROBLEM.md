# task-836 — Report field errors · closing marks removed (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length. Use FieldError::Missing for an empty field and FieldError::TooLong above 6 bytes.

```rust
pub fn closing_marks_removed_equals(text: &str) -> Result<usize, FieldError>
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `closing_marks_removed_equals("72;12")` gives `Ok(2)`.
- `closing_marks_removed_equals("猫=12;note")` gives `Ok(6)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-836/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-836`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
