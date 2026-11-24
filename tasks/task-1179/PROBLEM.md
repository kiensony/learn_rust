# task-1179 — Borrow a view · longest field with last tie (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller. Whenever the selected text is empty, return the caller’s fallback slice.

```rust
pub fn longest_field_with_last_tie_pipe<'a>(text: &'a str, fallback: &'a str) -> &'a str
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `longest_field_with_last_tie_pipe("first\n123", "backup")` gives `"123"`.
- `longest_field_with_last_tie_pipe("hi|kit", "backup")` gives `"kit"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1179/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1179`.

[Module reading](https://doc.rust-lang.org/reference/trait-bounds.html)
