# task-473 — Split a slice · remainder after first mark (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Inspect the longer slice in the pair (first on a tie). Return the selected field’s byte length.

```rust
pub fn remainder_after_first_mark_slash(fields: (&str, &str)) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_slash(("path/1/12", "x"))` gives `2`.
- `remainder_after_first_mark_slash(("path/123", "z/w"))` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-473/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-473`.

[Module reading](https://doc.rust-lang.org/book/ch04-03-slices.html)
