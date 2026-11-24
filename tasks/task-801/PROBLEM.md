# task-801 — Index duplicate fields · longest field with last tie (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Select one field from every record and count occurrences in a BTreeMap. Keep the empty key if it appears. Keys own their text.

```rust
pub fn longest_field_with_last_tie_hash(records: &[&str]) -> std::collections::BTreeMap<String, usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `longest_field_with_last_tie_hash(&["outside[red#blue]ignored", "[猫###a]"])` gives `std::collections::BTreeMap::from([(String::from("blue"), 1usize), (String::from("猫"), 1usize)])`.
- `longest_field_with_last_tie_hash(&["outside[red#blue]ignored", "[猫###a]", "unclosed[cat", "[]"])` gives `std::collections::BTreeMap::from([(String::from("blue"), 1usize), (String::from("猫"), 1usize), (String::from(""), 2usize)])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-801/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-801`.

[Module reading](https://doc.rust-lang.org/std/collections/)
