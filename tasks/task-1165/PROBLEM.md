# task-1165 — Borrow a view · marks removed at both ends (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller. Whenever the selected text is empty, return the caller’s fallback slice.

```rust
pub fn marks_removed_at_both_ends_slash<'a>(text: &'a str, fallback: &'a str) -> &'a str
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `marks_removed_at_both_ends_slash("path/1/12", "backup")` gives `"1/12"`.
- `marks_removed_at_both_ends_slash("path/123", "backup")` gives `"123"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1165/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1165`.

[Module reading](https://doc.rust-lang.org/reference/trait-bounds.html)
