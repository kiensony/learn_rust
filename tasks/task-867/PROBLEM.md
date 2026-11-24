# task-867 — Expose a module function · marks removed at both ends (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the field’s byte length including the header size in wire::HEADER_BYTES. Keep wire private. Add a rustdoc example exercising the public function.

```rust
pub fn marks_removed_at_both_ends_slash(text: &str) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `marks_removed_at_both_ends_slash("path/1/12")` gives `6`.
- `marks_removed_at_both_ends_slash("path/123")` gives `5`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-867/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-867`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
