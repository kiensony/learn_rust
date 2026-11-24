# task-948 — Call a supplied closure · remainder after first mark (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn remainder_after_first_mark_hash(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `remainder_after_first_mark_hash("outside[red#blue]ignored", |s| s.chars().count() + 1)` gives `5`.
- `remainder_after_first_mark_hash("[猫###a]", |s| s.len() * 2)` gives `6`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-948/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-948`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
