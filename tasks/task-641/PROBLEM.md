# task-641 — Read a record · remainder after first mark (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn remainder_after_first_mark_equals(record: &Record) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_equals(&Record { label: String::from("72;12"), revision: 0 })` gives `0`.
- `remainder_after_first_mark_equals(&Record { label: String::from("猫=12;note"), revision: 1 })` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-641/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-641`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
