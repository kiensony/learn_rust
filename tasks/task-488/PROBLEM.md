# task-488 — Split a slice · leading field (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Inspect the longer slice in the pair (first on a tie). Return the selected field’s byte length.

```rust
pub fn leading_field_equals(fields: (&str, &str)) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_equals(("72;12", "x"))` gives `2`.
- `leading_field_equals(("猫=12;note", "z=w"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-488/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-488`.

[Module reading](https://doc.rust-lang.org/book/ch04-03-slices.html)
