# task-937 — Call a supplied closure · marks removed at both ends (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn marks_removed_at_both_ends_equals(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `marks_removed_at_both_ends_equals("72;12", |s| s.chars().count() + 1)` gives `3`.
- `marks_removed_at_both_ends_equals("猫=12;note", |s| s.len() * 2)` gives `12`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-937/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-937`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
