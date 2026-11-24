# task-847 — Report field errors · marks removed at both ends (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length. Use FieldError::Missing for an empty field and FieldError::TooLong above 7 bytes.

```rust
pub fn marks_removed_at_both_ends_hash(text: &str) -> Result<usize, FieldError>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `marks_removed_at_both_ends_hash("outside[red#blue]ignored")` gives `Err(FieldError::TooLong)`.
- `marks_removed_at_both_ends_hash("[猫###a]")` gives `Ok(7)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-847/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-847`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
