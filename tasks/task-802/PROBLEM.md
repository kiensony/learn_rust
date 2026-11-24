# task-802 — Index duplicate fields · shortest field with first tie (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Select one field from every record and count occurrences in a BTreeMap. Keep the empty key if it appears. Keys own their text.

```rust
pub fn shortest_field_with_first_tie_hash(records: &[&str]) -> std::collections::BTreeMap<String, usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `shortest_field_with_first_tie_hash(&["outside[red#blue]ignored", "[猫###a]"])` gives `std::collections::BTreeMap::from([(String::from("red"), 1usize), (String::from(""), 1usize)])`.
- `shortest_field_with_first_tie_hash(&["outside[red#blue]ignored", "[猫###a]", "unclosed[cat", "[]"])` gives `std::collections::BTreeMap::from([(String::from("red"), 1usize), (String::from(""), 3usize)])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-802/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-802`.

[Module reading](https://doc.rust-lang.org/std/collections/)
