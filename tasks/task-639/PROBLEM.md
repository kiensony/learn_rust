# task-639 — Read a record · closing marks removed (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn closing_marks_removed_equals(record: &Record) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `closing_marks_removed_equals(&Record { label: String::from("72;12"), revision: 0 })` gives `2`.
- `closing_marks_removed_equals(&Record { label: String::from("猫=12;note"), revision: 1 })` gives `7`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-639/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-639`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
