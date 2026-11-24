# task-946 — Call a supplied closure · closing marks removed (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn closing_marks_removed_hash(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `closing_marks_removed_hash("outside[red#blue]ignored", |s| s.chars().count() + 1)` gives `9`.
- `closing_marks_removed_hash("[猫###a]", |s| s.len() * 2)` gives `14`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-946/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-946`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
