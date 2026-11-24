# task-735 — Decode an optional field · closing field (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return Some(value) for a selected field of one or more ASCII digits fitting a u16. Missing records, signs, whitespace, empty fields and overflow yield None.

```rust
pub fn closing_field_equals(record: Option<&str>) -> Option<u16>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_field_equals(Some("72;12"))` gives `Some(72)`.
- `closing_field_equals(Some("猫=12;note"))` gives `Some(12)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-735/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-735`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
