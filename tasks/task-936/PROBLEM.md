# task-936 — Call a supplied closure · closing marks removed (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn closing_marks_removed_equals(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `closing_marks_removed_equals("72;12", |s| s.chars().count() + 1)` gives `3`.
- `closing_marks_removed_equals("猫=12;note", |s| s.len() * 2)` gives `12`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-936/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-936`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
