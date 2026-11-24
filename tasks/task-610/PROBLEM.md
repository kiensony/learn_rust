# task-610 — Read a record · closing marks removed (:)

Treat ':' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn closing_marks_removed_colon(record: &Record) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_marks_removed_colon(&Record { label: String::from("hi:kit"), revision: 0 })` gives `6`.
- `closing_marks_removed_colon(&Record { label: String::from(""), revision: 2 })` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-610/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-610`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
