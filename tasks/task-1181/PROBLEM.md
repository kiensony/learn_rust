# task-1181 — Borrow a view · leading field (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller. Whenever the selected text is empty, return the caller’s fallback slice.

```rust
pub fn leading_field_equals<'a>(text: &'a str, fallback: &'a str) -> &'a str
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_equals("72;12", "backup")` gives `"72"`.
- `leading_field_equals("猫=12;note", "backup")` gives `"猫"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1181/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1181`.

[Module reading](https://doc.rust-lang.org/reference/trait-bounds.html)
