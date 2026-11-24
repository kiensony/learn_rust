# task-943 — Call a supplied closure · leading field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn leading_field_hash(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_hash("outside[red#blue]ignored", |s| s.chars().count() + 1)` gives `4`.
- `leading_field_hash("[猫###a]", |s| s.len() * 2)` gives `6`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-943/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-943`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
