# task-1184 — Borrow a view · closing marks removed (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller. Whenever the selected text is empty, return the caller’s fallback slice.

```rust
pub fn closing_marks_removed_equals<'a>(text: &'a str, fallback: &'a str) -> &'a str
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `closing_marks_removed_equals("72;12", "backup")` gives `"72"`.
- `closing_marks_removed_equals("猫=12;note", "backup")` gives `"猫=12"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1184/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1184`.

[Module reading](https://doc.rust-lang.org/reference/trait-bounds.html)
