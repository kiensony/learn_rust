# task-869 — Expose a module function · prefix before final mark (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the field’s byte length including the header size in wire::HEADER_BYTES. Keep wire private. Add a rustdoc example exercising the public function.

```rust
pub fn prefix_before_final_mark_slash(text: &str) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `prefix_before_final_mark_slash("path/1/12")` gives `3`.
- `prefix_before_final_mark_slash("path/123")` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-869/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-869`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
