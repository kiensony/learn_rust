# task-833 — Report field errors · leading field (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length. Use FieldError::Missing for an empty field and FieldError::TooLong above 6 bytes.

```rust
pub fn leading_field_equals(text: &str) -> Result<usize, FieldError>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_equals("72;12")` gives `Ok(2)`.
- `leading_field_equals("猫=12;note")` gives `Ok(3)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-833/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-833`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
