# task-653 — Read a record · first nonempty field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn first_nonempty_field_hash(record: &Record) -> usize
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `first_nonempty_field_hash(&Record { label: String::from("outside[red#blue]ignored"), revision: 0 })` gives `3`.
- `first_nonempty_field_hash(&Record { label: String::from("[猫###a]"), revision: 1 })` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-653/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-653`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
