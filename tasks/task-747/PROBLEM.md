# task-747 — Decode an optional field · closing marks removed (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return Some(value) for a selected field of one or more ASCII digits fitting a u16. Missing records, signs, whitespace, empty fields and overflow yield None.

```rust
pub fn closing_marks_removed_hash(record: Option<&str>) -> Option<u16>
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `closing_marks_removed_hash(Some("outside[red#blue]ignored"))` gives `None`.
- `closing_marks_removed_hash(Some("[42]"))` gives `Some(42)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-747/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-747`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
