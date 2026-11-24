# task-878 — Expose a module function · remainder after first mark (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the field’s byte length including the header size in wire::HEADER_BYTES. Keep wire private. Add a rustdoc example exercising the public function.

```rust
pub fn remainder_after_first_mark_pipe(text: &str) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_pipe("first\n123")` gives `2`.
- `remainder_after_first_mark_pipe("header\n猫|123")` gives `5`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-878/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-878`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
