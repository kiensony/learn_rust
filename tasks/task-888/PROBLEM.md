# task-888 — Expose a module function · remainder after first mark (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the field’s byte length including the header size in wire::HEADER_BYTES. Keep wire private. Add a rustdoc example exercising the public function.

```rust
pub fn remainder_after_first_mark_equals(text: &str) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_equals("72;12")` gives `2`.
- `remainder_after_first_mark_equals("猫=12;note")` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-888/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-888`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
