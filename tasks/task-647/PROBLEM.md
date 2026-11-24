# task-647 — Read a record · closing field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn closing_field_hash(record: &Record) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_field_hash(&Record { label: String::from("outside[red#blue]ignored"), revision: 0 })` gives `4`.
- `closing_field_hash(&Record { label: String::from("[猫###a]"), revision: 1 })` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-647/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-647`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
