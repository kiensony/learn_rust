# task-711 — Decode an optional field · first nonempty field (:)

Treat ':' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return Some(value) for a selected field of one or more ASCII digits fitting a u16. Missing records, signs, whitespace, empty fields and overflow yield None.

```rust
pub fn first_nonempty_field_colon(record: Option<&str>) -> Option<u16>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `first_nonempty_field_colon(Some("hi:kit"))` gives `None`.
- `first_nonempty_field_colon(Some("3:72"))` gives `Some(3)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-711/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-711`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
