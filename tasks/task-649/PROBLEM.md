# task-649 — Read a record · closing marks removed (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn closing_marks_removed_hash(record: &Record) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `closing_marks_removed_hash(&Record { label: String::from("outside[red#blue]ignored"), revision: 0 })` gives `8`.
- `closing_marks_removed_hash(&Record { label: String::from("unclosed[cat"), revision: 2 })` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-649/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-649`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
