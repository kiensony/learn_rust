# task-911 — Call a supplied closure · longest field with last tie (:)

Treat ':' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn longest_field_with_last_tie_colon(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `longest_field_with_last_tie_colon("hi:kit", |s| s.chars().count() + 1)` gives `4`.
- `longest_field_with_last_tie_colon("", |s| s.chars().count() + 1)` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-911/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-911`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
