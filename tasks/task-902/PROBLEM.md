# task-902 — Expose a module function · shortest field with first tie (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the field’s byte length including the header size in wire::HEADER_BYTES. Keep wire private. Add a rustdoc example exercising the public function.

```rust
pub fn shortest_field_with_first_tie_hash(text: &str) -> usize
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `shortest_field_with_first_tie_hash("outside[red#blue]ignored")` gives `5`.
- `shortest_field_with_first_tie_hash("[猫###a]")` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-902/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-902`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
