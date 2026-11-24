# task-835 — Report field errors · leading marks removed (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length. Use FieldError::Missing for an empty field and FieldError::TooLong above 6 bytes.

```rust
pub fn leading_marks_removed_equals(text: &str) -> Result<usize, FieldError>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `leading_marks_removed_equals("72;12")` gives `Ok(2)`.
- `leading_marks_removed_equals("猫=12;note")` gives `Ok(6)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-835/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-835`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
