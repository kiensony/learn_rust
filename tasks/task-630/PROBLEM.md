# task-630 — Read a record · marks removed at both ends (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn marks_removed_at_both_ends_pipe(record: &Record) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `marks_removed_at_both_ends_pipe(&Record { label: String::from("first\n123"), revision: 0 })` gives `3`.
- `marks_removed_at_both_ends_pipe(&Record { label: String::from("header\n猫|123"), revision: 1 })` gives `8`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-630/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-630`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
