# task-648 — Read a record · leading marks removed (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn leading_marks_removed_hash(record: &Record) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `leading_marks_removed_hash(&Record { label: String::from("outside[red#blue]ignored"), revision: 0 })` gives `8`.
- `leading_marks_removed_hash(&Record { label: String::from("unclosed[cat"), revision: 2 })` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-648/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-648`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
