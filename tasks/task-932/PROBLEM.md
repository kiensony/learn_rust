# task-932 — Call a supplied closure · shortest field with first tie (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn shortest_field_with_first_tie_pipe(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `shortest_field_with_first_tie_pipe("first\n123", |s| s.chars().count() + 1)` gives `4`.
- `shortest_field_with_first_tie_pipe("header\n猫|123", |s| s.len() * 2)` gives `6`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-932/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-932`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
