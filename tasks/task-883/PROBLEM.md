# task-883 — Expose a module function · leading field (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the field’s byte length including the header size in wire::HEADER_BYTES. Keep wire private. Add a rustdoc example exercising the public function.

```rust
pub fn leading_field_equals(text: &str) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_equals("72;12")` gives `4`.
- `leading_field_equals("猫=12;note")` gives `5`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-883/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-883`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
