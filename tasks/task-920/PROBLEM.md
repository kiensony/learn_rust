# task-920 — Call a supplied closure · first nonempty field (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn first_nonempty_field_slash(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `first_nonempty_field_slash("path/1/12", |s| s.chars().count() + 1)` gives `2`.
- `first_nonempty_field_slash("path/123", |s| s.len() * 2)` gives `6`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-920/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-920`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
