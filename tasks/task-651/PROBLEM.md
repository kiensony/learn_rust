# task-651 — Read a record · remainder after first mark (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn remainder_after_first_mark_hash(record: &Record) -> usize
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `remainder_after_first_mark_hash(&Record { label: String::from("outside[red#blue]ignored"), revision: 0 })` gives `4`.
- `remainder_after_first_mark_hash(&Record { label: String::from("unclosed[cat"), revision: 2 })` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-651/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-651`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
