# task-493 — Split a slice · remainder after first mark (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Inspect the longer slice in the pair (first on a tie). Return the selected field’s byte length.

```rust
pub fn remainder_after_first_mark_equals(fields: (&str, &str)) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_equals(("72;12", "x"))` gives `0`.
- `remainder_after_first_mark_equals(("猫=12;note", "z=w"))` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-493/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-493`.

[Module reading](https://doc.rust-lang.org/book/ch04-03-slices.html)
