# task-613 — Read a record · first nonempty field (:)

Treat ':' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn first_nonempty_field_colon(record: &Record) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `first_nonempty_field_colon(&Record { label: String::from("hi:kit"), revision: 0 })` gives `2`.
- `first_nonempty_field_colon(&Record { label: String::from(":é:a"), revision: 1 })` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-613/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-613`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
