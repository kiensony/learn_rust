# task-628 — Read a record · leading marks removed (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn leading_marks_removed_pipe(record: &Record) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_marks_removed_pipe(&Record { label: String::from("first\n123"), revision: 0 })` gives `3`.
- `leading_marks_removed_pipe(&Record { label: String::from("header\n猫|123"), revision: 1 })` gives `8`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-628/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-628`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
