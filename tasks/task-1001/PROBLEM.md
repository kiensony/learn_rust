# task-1001 — Iterate over matching fields · shortest field with first tie (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Produce a Vec of one selected byte length per record in input order using an iterator adapter. Keep duplicates and zero lengths.

```rust
pub fn shortest_field_with_first_tie_hash<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `shortest_field_with_first_tie_hash(vec!["outside[red#blue]ignored", "[猫###a]"])` gives `vec![3usize, 0usize]`.
- `shortest_field_with_first_tie_hash(vec!["[猫###a]", "unclosed[cat"])` gives `vec![0usize, 0usize]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1001/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1001`.

[Module reading](https://doc.rust-lang.org/book/ch13-02-iterators.html)
