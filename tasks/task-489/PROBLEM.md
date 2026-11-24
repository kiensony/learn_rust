# task-489 — Split a slice · closing field (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Inspect the longer slice in the pair (first on a tie). Return the selected field’s byte length.

```rust
pub fn closing_field_equals(fields: (&str, &str)) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_field_equals(("72;12", "x"))` gives `2`.
- `closing_field_equals(("hi=kit", "x"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-489/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-489`.

[Module reading](https://doc.rust-lang.org/book/ch04-03-slices.html)
