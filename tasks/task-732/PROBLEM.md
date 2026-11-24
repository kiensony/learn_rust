# task-732 — Decode an optional field · longest field with last tie (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return Some(value) for a selected field of one or more ASCII digits fitting a u16. Missing records, signs, whitespace, empty fields and overflow yield None.

```rust
pub fn longest_field_with_last_tie_pipe(record: Option<&str>) -> Option<u16>
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `longest_field_with_last_tie_pipe(Some("first\n123"))` gives `Some(123)`.
- `longest_field_with_last_tie_pipe(Some("hi|kit"))` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-732/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-732`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
