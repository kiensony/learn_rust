# task-611 — Read a record · marks removed at both ends (:)

Treat ':' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn marks_removed_at_both_ends_colon(record: &Record) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `marks_removed_at_both_ends_colon(&Record { label: String::from("hi:kit"), revision: 0 })` gives `6`.
- `marks_removed_at_both_ends_colon(&Record { label: String::from(":é:a"), revision: 1 })` gives `5`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-611/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-611`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
