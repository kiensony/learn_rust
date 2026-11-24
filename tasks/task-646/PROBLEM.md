# task-646 — Read a record · leading field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn leading_field_hash(record: &Record) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_hash(&Record { label: String::from("outside[red#blue]ignored"), revision: 0 })` gives `3`.
- `leading_field_hash(&Record { label: String::from("[猫###a]"), revision: 1 })` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-646/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-646`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
