# task-866 — Expose a module function · closing marks removed (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the field’s byte length including the header size in wire::HEADER_BYTES. Keep wire private. Add a rustdoc example exercising the public function.

```rust
pub fn closing_marks_removed_slash(text: &str) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_marks_removed_slash("path/1/12")` gives `6`.
- `closing_marks_removed_slash("path/123")` gives `5`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-866/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-866`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
