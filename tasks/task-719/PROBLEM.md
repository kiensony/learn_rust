# task-719 — Decode an optional field · remainder after first mark (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return Some(value) for a selected field of one or more ASCII digits fitting a u16. Missing records, signs, whitespace, empty fields and overflow yield None.

```rust
pub fn remainder_after_first_mark_slash(record: Option<&str>) -> Option<u16>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_slash(Some("path/1/12"))` gives `Some(12)`.
- `remainder_after_first_mark_slash(Some("path/123"))` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-719/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-719`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
