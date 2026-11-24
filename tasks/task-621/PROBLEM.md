# task-621 — Read a record · remainder after first mark (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn remainder_after_first_mark_slash(record: &Record) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_slash(&Record { label: String::from("path/1/12"), revision: 0 })` gives `2`.
- `remainder_after_first_mark_slash(&Record { label: String::from("path/123"), revision: 1 })` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-621/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-621`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
