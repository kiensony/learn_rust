# task-751 — Decode an optional field · first nonempty field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return Some(value) for a selected field of one or more ASCII digits fitting a u16. Missing records, signs, whitespace, empty fields and overflow yield None.

```rust
pub fn first_nonempty_field_hash(record: Option<&str>) -> Option<u16>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `first_nonempty_field_hash(Some("outside[red#blue]ignored"))` gives `None`.
- `first_nonempty_field_hash(Some("[42]"))` gives `Some(42)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-751/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-751`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
