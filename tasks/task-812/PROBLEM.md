# task-812 — Report field errors · shortest field with first tie (:)

Treat ':' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length. Use FieldError::Missing for an empty field and FieldError::TooLong above 3 bytes.

```rust
pub fn shortest_field_with_first_tie_colon(text: &str) -> Result<usize, FieldError>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shortest_field_with_first_tie_colon("hi:kit")` gives `Ok(2)`.
- `shortest_field_with_first_tie_colon(":é:a")` gives `Err(FieldError::Missing)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-812/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-812`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
