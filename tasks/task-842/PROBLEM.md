# task-842 — Report field errors · shortest field with first tie (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length. Use FieldError::Missing for an empty field and FieldError::TooLong above 6 bytes.

```rust
pub fn shortest_field_with_first_tie_equals(text: &str) -> Result<usize, FieldError>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `shortest_field_with_first_tie_equals("72;12")` gives `Ok(2)`.
- `shortest_field_with_first_tie_equals("=é=a")` gives `Err(FieldError::Missing)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-842/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-842`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
