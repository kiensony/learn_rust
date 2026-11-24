# task-808 — Report field errors · remainder after first mark (:)

Treat ':' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length. Use FieldError::Missing for an empty field and FieldError::TooLong above 3 bytes.

```rust
pub fn remainder_after_first_mark_colon(text: &str) -> Result<usize, FieldError>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_colon("hi:kit")` gives `Ok(3)`.
- `remainder_after_first_mark_colon(":é:a")` gives `Err(FieldError::TooLong)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-808/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-808`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
