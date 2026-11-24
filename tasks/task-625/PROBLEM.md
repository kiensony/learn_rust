# task-625 — Read a record · shortest field with first tie (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn shortest_field_with_first_tie_slash(record: &Record) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shortest_field_with_first_tie_slash(&Record { label: String::from("path/1/12"), revision: 0 })` gives `1`.
- `shortest_field_with_first_tie_slash(&Record { label: String::from("path/123"), revision: 1 })` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-625/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-625`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
