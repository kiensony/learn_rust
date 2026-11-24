# task-738 — Decode an optional field · marks removed at both ends (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return Some(value) for a selected field of one or more ASCII digits fitting a u16. Missing records, signs, whitespace, empty fields and overflow yield None.

```rust
pub fn marks_removed_at_both_ends_equals(record: Option<&str>) -> Option<u16>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `marks_removed_at_both_ends_equals(Some("72;12"))` gives `Some(72)`.
- `marks_removed_at_both_ends_equals(Some("猫=12;note"))` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-738/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-738`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
