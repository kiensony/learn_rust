# task-739 — Decode an optional field · remainder after first mark (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return Some(value) for a selected field of one or more ASCII digits fitting a u16. Missing records, signs, whitespace, empty fields and overflow yield None.

```rust
pub fn remainder_after_first_mark_equals(record: Option<&str>) -> Option<u16>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_equals(Some("72;12"))` gives `None`.
- `remainder_after_first_mark_equals(Some("猫=12;note"))` gives `Some(12)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-739/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-739`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
