# task-1051 — Accept generic text · shortest field with first tie (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept both String and &str elements through AsRef<str>. Return the selected byte lengths in an array of the same compile-time length.

```rust
pub fn shortest_field_with_first_tie_hash<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N]
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `shortest_field_with_first_tie_hash(["outside[red#blue]ignored", "[猫###a]"])` gives `[3, 0]`.
- `shortest_field_with_first_tie_hash([String::from("[猫###a]"), String::from("unclosed[cat")])` gives `[0, 0]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1051/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1051`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
