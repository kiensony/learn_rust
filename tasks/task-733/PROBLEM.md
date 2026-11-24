# task-733 — Decode an optional field · shortest field with first tie (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return Some(value) for a selected field of one or more ASCII digits fitting a u16. Missing records, signs, whitespace, empty fields and overflow yield None.

```rust
pub fn shortest_field_with_first_tie_pipe(record: Option<&str>) -> Option<u16>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `shortest_field_with_first_tie_pipe(Some("first\n123"))` gives `Some(123)`.
- `shortest_field_with_first_tie_pipe(Some("header\n猫|123"))` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-733/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-733`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
