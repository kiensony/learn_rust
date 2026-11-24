# task-1150 — Read a trait object · shortest field with first tie (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Read the field through a dynamically dispatched Source. Return its byte length for any concrete implementor. Explain which method is selected at runtime.

```rust
pub fn shortest_field_with_first_tie_hash(source: &dyn Source) -> usize
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `shortest_field_with_first_tie_hash(&"outside[red#blue]ignored")` gives `3`.
- `shortest_field_with_first_tie_hash(&String::from("[猫###a]"))` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1150/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1150`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)
