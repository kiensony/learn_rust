# task-811 — Report field errors · longest field with last tie (:)

Treat ':' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length. Use FieldError::Missing for an empty field and FieldError::TooLong above 3 bytes.

```rust
pub fn longest_field_with_last_tie_colon(text: &str) -> Result<usize, FieldError>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `longest_field_with_last_tie_colon("hi:kit")` gives `Ok(3)`.
- `longest_field_with_last_tie_colon(":é:a")` gives `Ok(2)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-811/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-811`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
