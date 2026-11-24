# task-357 — Consume a label · shortest field with first tie (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Take ownership of the input and return the selected text as an owned String. The caller must be able to move the result into a new binding.

```rust
pub fn shortest_field_with_first_tie_hash(mut text: String) -> String
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `shortest_field_with_first_tie_hash(String::from("outside[red#blue]ignored"))` gives `"red"`.
- `shortest_field_with_first_tie_hash(String::from("[猫###a]"))` gives `""`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-357/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-357`.

[Module reading](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
