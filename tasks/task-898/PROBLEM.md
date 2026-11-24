# task-898 — Expose a module function · remainder after first mark (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the field’s byte length including the header size in wire::HEADER_BYTES. Keep wire private. Add a rustdoc example exercising the public function.

```rust
pub fn remainder_after_first_mark_hash(text: &str) -> usize
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `remainder_after_first_mark_hash("outside[red#blue]ignored")` gives `6`.
- `remainder_after_first_mark_hash("[猫###a]")` gives `5`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-898/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-898`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
