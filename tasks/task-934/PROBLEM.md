# task-934 — Call a supplied closure · closing field (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn closing_field_equals(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_field_equals("72;12", |s| s.chars().count() + 1)` gives `3`.
- `closing_field_equals("猫=12;note", |s| s.len() * 2)` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-934/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-934`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
