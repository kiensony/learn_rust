# task-1154 — Borrow a view · closing marks removed (:)

Treat ':' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller. Whenever the selected text is empty, return the caller’s fallback slice.

```rust
pub fn closing_marks_removed_colon<'a>(text: &'a str, fallback: &'a str) -> &'a str
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_marks_removed_colon("hi:kit", "backup")` gives `"hi:kit"`.
- `closing_marks_removed_colon(":é:a", "backup")` gives `":é:a"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1154/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1154`.

[Module reading](https://doc.rust-lang.org/reference/trait-bounds.html)
