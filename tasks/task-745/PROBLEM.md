# task-745 — Decode an optional field · closing field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return Some(value) for a selected field of one or more ASCII digits fitting a u16. Missing records, signs, whitespace, empty fields and overflow yield None.

```rust
pub fn closing_field_hash(record: Option<&str>) -> Option<u16>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_field_hash(Some("outside[red#blue]ignored"))` gives `None`.
- `closing_field_hash(Some("[42]"))` gives `Some(42)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-745/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-745`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
