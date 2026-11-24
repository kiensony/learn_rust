# task-901 — Expose a module function · longest field with last tie (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the field’s byte length including the header size in wire::HEADER_BYTES. Keep wire private. Add a rustdoc example exercising the public function.

```rust
pub fn longest_field_with_last_tie_hash(text: &str) -> usize
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `longest_field_with_last_tie_hash("outside[red#blue]ignored")` gives `6`.
- `longest_field_with_last_tie_hash("[猫###a]")` gives `5`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-901/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-901`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
