# task-853 — Expose a module function · leading field (:)

Treat ':' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the field’s byte length including the header size in wire::HEADER_BYTES. Keep wire private. Add a rustdoc example exercising the public function.

```rust
pub fn leading_field_colon(text: &str) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_colon("hi:kit")` gives `4`.
- `leading_field_colon(":é:a")` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-853/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-853`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
