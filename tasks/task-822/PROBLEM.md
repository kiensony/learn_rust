# task-822 — Report field errors · shortest field with first tie (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length. Use FieldError::Missing for an empty field and FieldError::TooLong above 4 bytes.

```rust
pub fn shortest_field_with_first_tie_slash(text: &str) -> Result<usize, FieldError>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shortest_field_with_first_tie_slash("path/1/12")` gives `Ok(1)`.
- `shortest_field_with_first_tie_slash("path/123")` gives `Ok(3)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-822/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-822`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
