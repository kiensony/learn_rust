# task-894 — Expose a module function · closing field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the field’s byte length including the header size in wire::HEADER_BYTES. Keep wire private. Add a rustdoc example exercising the public function.

```rust
pub fn closing_field_hash(text: &str) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_field_hash("outside[red#blue]ignored")` gives `6`.
- `closing_field_hash("[猫###a]")` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-894/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-894`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
