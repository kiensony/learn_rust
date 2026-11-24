# task-1190 — Borrow a view · shortest field with first tie (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller. Whenever the selected text is empty, return the caller’s fallback slice.

```rust
pub fn shortest_field_with_first_tie_equals<'a>(text: &'a str, fallback: &'a str) -> &'a str
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `shortest_field_with_first_tie_equals("72;12", "backup")` gives `"72"`.
- `shortest_field_with_first_tie_equals("猫=12;note", "backup")` gives `"12"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1190/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1190`.

[Module reading](https://doc.rust-lang.org/reference/trait-bounds.html)
