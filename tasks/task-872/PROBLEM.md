# task-872 — Expose a module function · shortest field with first tie (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the field’s byte length including the header size in wire::HEADER_BYTES. Keep wire private. Add a rustdoc example exercising the public function.

```rust
pub fn shortest_field_with_first_tie_slash(text: &str) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shortest_field_with_first_tie_slash("path/1/12")` gives `3`.
- `shortest_field_with_first_tie_slash("path/123")` gives `5`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-872/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-872`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
