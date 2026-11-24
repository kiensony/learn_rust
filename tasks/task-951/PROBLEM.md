# task-951 — Call a supplied closure · longest field with last tie (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn longest_field_with_last_tie_hash(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `longest_field_with_last_tie_hash("outside[red#blue]ignored", |s| s.chars().count() + 1)` gives `5`.
- `longest_field_with_last_tie_hash("[猫###a]", |s| s.len() * 2)` gives `6`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-951/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-951`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
