# task-710 — Decode an optional field · prefix before final mark (:)

Treat ':' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return Some(value) for a selected field of one or more ASCII digits fitting a u16. Missing records, signs, whitespace, empty fields and overflow yield None.

```rust
pub fn prefix_before_final_mark_colon(record: Option<&str>) -> Option<u16>
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `prefix_before_final_mark_colon(Some("hi:kit"))` gives `None`.
- `prefix_before_final_mark_colon(Some("3:72"))` gives `Some(3)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-710/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-710`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
