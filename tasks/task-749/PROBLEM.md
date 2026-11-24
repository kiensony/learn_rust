# task-749 — Decode an optional field · remainder after first mark (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return Some(value) for a selected field of one or more ASCII digits fitting a u16. Missing records, signs, whitespace, empty fields and overflow yield None.

```rust
pub fn remainder_after_first_mark_hash(record: Option<&str>) -> Option<u16>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `remainder_after_first_mark_hash(Some("outside[red#12]ignored"))` gives `Some(12)`.
- `remainder_after_first_mark_hash(Some("[猫###a]"))` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-749/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-749`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
