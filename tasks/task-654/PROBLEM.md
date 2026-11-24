# task-654 — Read a record · longest field with last tie (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn longest_field_with_last_tie_hash(record: &Record) -> usize
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `longest_field_with_last_tie_hash(&Record { label: String::from("outside[red#blue]ignored"), revision: 0 })` gives `4`.
- `longest_field_with_last_tie_hash(&Record { label: String::from("unclosed[cat"), revision: 2 })` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-654/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-654`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
