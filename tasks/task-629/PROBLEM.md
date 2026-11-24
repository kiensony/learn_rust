# task-629 — Read a record · closing marks removed (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn closing_marks_removed_pipe(record: &Record) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `closing_marks_removed_pipe(&Record { label: String::from("first\n123"), revision: 0 })` gives `3`.
- `closing_marks_removed_pipe(&Record { label: String::from("header\n猫|123"), revision: 1 })` gives `8`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-629/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-629`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
