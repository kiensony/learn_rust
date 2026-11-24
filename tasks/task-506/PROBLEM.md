# task-506 — Split a slice · longest field with last tie (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Inspect the longer slice in the pair (first on a tie). Return the selected field’s byte length.

```rust
pub fn longest_field_with_last_tie_hash(fields: (&str, &str)) -> usize
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `longest_field_with_last_tie_hash(("outside[red#blue]ignored", "x"))` gives `4`.
- `longest_field_with_last_tie_hash(("[猫###a]", "z#w"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-506/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-506`.

[Module reading](https://doc.rust-lang.org/book/ch04-03-slices.html)
