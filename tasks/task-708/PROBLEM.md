# task-708 — Decode an optional field · closing marks removed (:)

Treat ':' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return Some(value) for a selected field of one or more ASCII digits fitting a u16. Missing records, signs, whitespace, empty fields and overflow yield None.

```rust
pub fn closing_marks_removed_colon(record: Option<&str>) -> Option<u16>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_marks_removed_colon(Some("hi:kit"))` gives `None`.
- `closing_marks_removed_colon(Some("3300"))` gives `Some(3300)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-708/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-708`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
