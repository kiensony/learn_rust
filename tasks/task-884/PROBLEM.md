# task-884 — Expose a module function · closing field (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the field’s byte length including the header size in wire::HEADER_BYTES. Keep wire private. Add a rustdoc example exercising the public function.

```rust
pub fn closing_field_equals(text: &str) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_field_equals("72;12")` gives `4`.
- `closing_field_equals("hi=kit")` gives `5`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-884/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-884`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
