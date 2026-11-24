# task-798 — Index duplicate fields · remainder after first mark (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Select one field from every record and count occurrences in a BTreeMap. Keep the empty key if it appears. Keys own their text.

```rust
pub fn remainder_after_first_mark_hash(records: &[&str]) -> std::collections::BTreeMap<String, usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `remainder_after_first_mark_hash(&["outside[red#blue]ignored", "[猫###a]"])` gives `std::collections::BTreeMap::from([(String::from("blue"), 1usize), (String::from("##a"), 1usize)])`.
- `remainder_after_first_mark_hash(&["outside[red#blue]ignored", "[猫###a]", "unclosed[cat", "[]"])` gives `std::collections::BTreeMap::from([(String::from("blue"), 1usize), (String::from("##a"), 1usize), (String::from(""), 2usize)])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-798/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-798`.

[Module reading](https://doc.rust-lang.org/std/collections/)
