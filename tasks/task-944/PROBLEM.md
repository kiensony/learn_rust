# task-944 — Call a supplied closure · closing field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn closing_field_hash(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_field_hash("outside[red#blue]ignored", |s| s.chars().count() + 1)` gives `5`.
- `closing_field_hash("[猫###a]", |s| s.len() * 2)` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-944/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-944`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
