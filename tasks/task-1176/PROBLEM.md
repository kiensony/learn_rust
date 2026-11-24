# task-1176 — Borrow a view · remainder after first mark (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller. Whenever the selected text is empty, return the caller’s fallback slice.

```rust
pub fn remainder_after_first_mark_pipe<'a>(text: &'a str, fallback: &'a str) -> &'a str
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_pipe("first\n123", "backup")` gives `"backup"`.
- `remainder_after_first_mark_pipe("header\n猫|123", "backup")` gives `"123"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1176/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1176`.

[Module reading](https://doc.rust-lang.org/reference/trait-bounds.html)
