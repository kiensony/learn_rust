# task-1172 — Borrow a view · closing field (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller. Whenever the selected text is empty, return the caller’s fallback slice.

```rust
pub fn closing_field_pipe<'a>(text: &'a str, fallback: &'a str) -> &'a str
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_field_pipe("first\n123", "backup")` gives `"123"`.
- `closing_field_pipe("hi|kit", "backup")` gives `"kit"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1172/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1172`.

[Module reading](https://doc.rust-lang.org/reference/trait-bounds.html)
