# task-839 — Report field errors · prefix before final mark (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length. Use FieldError::Missing for an empty field and FieldError::TooLong above 6 bytes.

```rust
pub fn prefix_before_final_mark_equals(text: &str) -> Result<usize, FieldError>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `prefix_before_final_mark_equals("72;12")` gives `Err(FieldError::Missing)`.
- `prefix_before_final_mark_equals("猫=12;note")` gives `Ok(3)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-839/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-839`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
