# task-1173 — Borrow a view · leading marks removed (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller. Whenever the selected text is empty, return the caller’s fallback slice.

```rust
pub fn leading_marks_removed_pipe<'a>(text: &'a str, fallback: &'a str) -> &'a str
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_marks_removed_pipe("first\n123", "backup")` gives `"123"`.
- `leading_marks_removed_pipe("header\n猫|123", "backup")` gives `"猫|123"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1173/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1173`.

[Module reading](https://doc.rust-lang.org/reference/trait-bounds.html)
