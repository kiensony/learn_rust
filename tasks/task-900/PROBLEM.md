# task-900 — Expose a module function · first nonempty field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the field’s byte length including the header size in wire::HEADER_BYTES. Keep wire private. Add a rustdoc example exercising the public function.

```rust
pub fn first_nonempty_field_hash(text: &str) -> usize
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `first_nonempty_field_hash("outside[red#blue]ignored")` gives `5`.
- `first_nonempty_field_hash("unclosed[cat")` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-900/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-900`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
