# task-950 — Call a supplied closure · first nonempty field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn first_nonempty_field_hash(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `first_nonempty_field_hash("outside[red#blue]ignored", |s| s.chars().count() + 1)` gives `4`.
- `first_nonempty_field_hash("[猫###a]", |s| s.len() * 2)` gives `6`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-950/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-950`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
