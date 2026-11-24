# task-804 — Report field errors · closing field (:)

Treat ':' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length. Use FieldError::Missing for an empty field and FieldError::TooLong above 3 bytes.

```rust
pub fn closing_field_colon(text: &str) -> Result<usize, FieldError>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_field_colon("hi:kit")` gives `Ok(3)`.
- `closing_field_colon(":é:a")` gives `Ok(1)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-804/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-804`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
