# task-715 — Decode an optional field · closing field (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return Some(value) for a selected field of one or more ASCII digits fitting a u16. Missing records, signs, whitespace, empty fields and overflow yield None.

```rust
pub fn closing_field_slash(record: Option<&str>) -> Option<u16>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `closing_field_slash(Some("path/1/12"))` gives `Some(12)`.
- `closing_field_slash(Some("path/123"))` gives `Some(123)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-715/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-715`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
