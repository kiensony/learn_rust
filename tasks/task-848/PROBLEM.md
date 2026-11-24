# task-848 — Report field errors · remainder after first mark (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length. Use FieldError::Missing for an empty field and FieldError::TooLong above 7 bytes.

```rust
pub fn remainder_after_first_mark_hash(text: &str) -> Result<usize, FieldError>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `remainder_after_first_mark_hash("outside[red#blue]ignored")` gives `Ok(4)`.
- `remainder_after_first_mark_hash("[猫###a]")` gives `Ok(3)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-848/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-848`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
