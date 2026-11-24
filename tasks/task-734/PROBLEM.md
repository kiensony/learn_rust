# task-734 — Decode an optional field · leading field (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return Some(value) for a selected field of one or more ASCII digits fitting a u16. Missing records, signs, whitespace, empty fields and overflow yield None.

```rust
pub fn leading_field_equals(record: Option<&str>) -> Option<u16>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_equals(Some("72;12"))` gives `Some(72)`.
- `leading_field_equals(Some("猫=12;note"))` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-734/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-734`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
