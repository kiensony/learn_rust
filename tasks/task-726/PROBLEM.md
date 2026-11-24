# task-726 — Decode an optional field · leading marks removed (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return Some(value) for a selected field of one or more ASCII digits fitting a u16. Missing records, signs, whitespace, empty fields and overflow yield None.

```rust
pub fn leading_marks_removed_pipe(record: Option<&str>) -> Option<u16>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_marks_removed_pipe(Some("first\n123"))` gives `Some(123)`.
- `leading_marks_removed_pipe(Some("header\n猫|123"))` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-726/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-726`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
