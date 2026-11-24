# task-843 — Report field errors · leading field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length. Use FieldError::Missing for an empty field and FieldError::TooLong above 7 bytes.

```rust
pub fn leading_field_hash(text: &str) -> Result<usize, FieldError>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_hash("outside[red#blue]ignored")` gives `Ok(3)`.
- `leading_field_hash("unclosed[cat")` gives `Err(FieldError::Missing)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-843/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-843`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
