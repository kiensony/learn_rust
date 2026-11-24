# task-617 — Read a record · closing field (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn closing_field_slash(record: &Record) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `closing_field_slash(&Record { label: String::from("path/1/12"), revision: 0 })` gives `2`.
- `closing_field_slash(&Record { label: String::from("path/123"), revision: 1 })` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-617/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-617`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
