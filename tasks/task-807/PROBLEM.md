# task-807 — Report field errors · marks removed at both ends (:)

Treat ':' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length. Use FieldError::Missing for an empty field and FieldError::TooLong above 3 bytes.

```rust
pub fn marks_removed_at_both_ends_colon(text: &str) -> Result<usize, FieldError>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `marks_removed_at_both_ends_colon("hi:kit")` gives `Err(FieldError::TooLong)`.
- `marks_removed_at_both_ends_colon("")` gives `Err(FieldError::Missing)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-807/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-807`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
