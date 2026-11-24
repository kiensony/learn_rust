# task-1159 — Borrow a view · longest field with last tie (:)

Treat ':' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller. Whenever the selected text is empty, return the caller’s fallback slice.

```rust
pub fn longest_field_with_last_tie_colon<'a>(text: &'a str, fallback: &'a str) -> &'a str
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `longest_field_with_last_tie_colon("hi:kit", "backup")` gives `"kit"`.
- `longest_field_with_last_tie_colon(":é:a", "backup")` gives `"é"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1159/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1159`.

[Module reading](https://doc.rust-lang.org/reference/trait-bounds.html)
