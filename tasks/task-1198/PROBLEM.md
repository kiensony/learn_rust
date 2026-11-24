# task-1198 — Borrow a view · first nonempty field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller. Whenever the selected text is empty, return the caller’s fallback slice.

```rust
pub fn first_nonempty_field_hash<'a>(text: &'a str, fallback: &'a str) -> &'a str
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `first_nonempty_field_hash("outside[red#blue]ignored", "backup")` gives `"red"`.
- `first_nonempty_field_hash("[猫###a]", "backup")` gives `"猫"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1198/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1198`.

[Module reading](https://doc.rust-lang.org/reference/trait-bounds.html)
