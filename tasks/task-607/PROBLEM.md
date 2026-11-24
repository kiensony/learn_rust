# task-607 — Read a record · leading field (:)

Treat ':' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn leading_field_colon(record: &Record) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_colon(&Record { label: String::from("hi:kit"), revision: 0 })` gives `2`.
- `leading_field_colon(&Record { label: String::from(":é:a"), revision: 1 })` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-607/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-607`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
