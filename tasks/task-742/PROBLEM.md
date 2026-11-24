# task-742 — Decode an optional field · longest field with last tie (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return Some(value) for a selected field of one or more ASCII digits fitting a u16. Missing records, signs, whitespace, empty fields and overflow yield None.

```rust
pub fn longest_field_with_last_tie_equals(record: Option<&str>) -> Option<u16>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `longest_field_with_last_tie_equals(Some("72;12"))` gives `Some(72)`.
- `longest_field_with_last_tie_equals(Some("猫=12;note"))` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-742/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-742`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
