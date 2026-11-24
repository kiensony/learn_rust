# task-713 — Decode an optional field · shortest field with first tie (:)

Treat ':' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return Some(value) for a selected field of one or more ASCII digits fitting a u16. Missing records, signs, whitespace, empty fields and overflow yield None.

```rust
pub fn shortest_field_with_first_tie_colon(record: Option<&str>) -> Option<u16>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shortest_field_with_first_tie_colon(Some("hi:kit"))` gives `None`.
- `shortest_field_with_first_tie_colon(Some("3:72"))` gives `Some(3)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-713/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-713`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
