# task-1192 — Borrow a view · closing field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller. Whenever the selected text is empty, return the caller’s fallback slice.

```rust
pub fn closing_field_hash<'a>(text: &'a str, fallback: &'a str) -> &'a str
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_field_hash("outside[red#blue]ignored", "backup")` gives `"blue"`.
- `closing_field_hash("[猫###a]", "backup")` gives `"a"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1192/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1192`.

[Module reading](https://doc.rust-lang.org/reference/trait-bounds.html)
