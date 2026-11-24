# task-800 — Index duplicate fields · first nonempty field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Select one field from every record and count occurrences in a BTreeMap. Keep the empty key if it appears. Keys own their text.

```rust
pub fn first_nonempty_field_hash(records: &[&str]) -> std::collections::BTreeMap<String, usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `first_nonempty_field_hash(&["outside[red#blue]ignored", "[猫###a]"])` gives `std::collections::BTreeMap::from([(String::from("red"), 1usize), (String::from("猫"), 1usize)])`.
- `first_nonempty_field_hash(&["outside[red#blue]ignored", "[猫###a]", "unclosed[cat", "[]"])` gives `std::collections::BTreeMap::from([(String::from("red"), 1usize), (String::from("猫"), 1usize), (String::from(""), 2usize)])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-800/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-800`.

[Module reading](https://doc.rust-lang.org/std/collections/)
