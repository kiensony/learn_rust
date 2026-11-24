# task-935 — Call a supplied closure · leading marks removed (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn leading_marks_removed_equals(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `leading_marks_removed_equals("72;12", |s| s.chars().count() + 1)` gives `3`.
- `leading_marks_removed_equals("猫=12;note", |s| s.len() * 2)` gives `12`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-935/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-935`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
