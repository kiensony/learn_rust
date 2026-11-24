# task-821 — Report field errors · longest field with last tie (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length. Use FieldError::Missing for an empty field and FieldError::TooLong above 4 bytes.

```rust
pub fn longest_field_with_last_tie_slash(text: &str) -> Result<usize, FieldError>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `longest_field_with_last_tie_slash("path/1/12")` gives `Ok(2)`.
- `longest_field_with_last_tie_slash("path/123")` gives `Ok(3)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-821/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-821`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
