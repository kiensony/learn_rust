# task-906 — Call a supplied closure · closing marks removed (:)

Treat ':' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn closing_marks_removed_colon(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_marks_removed_colon("hi:kit", |s| s.chars().count() + 1)` gives `7`.
- `closing_marks_removed_colon(":é:a", |s| s.len() * 2)` gives `10`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-906/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-906`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
