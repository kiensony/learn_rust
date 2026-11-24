# task-476 — Split a slice · longest field with last tie (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Inspect the longer slice in the pair (first on a tie). Return the selected field’s byte length.

```rust
pub fn longest_field_with_last_tie_slash(fields: (&str, &str)) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `longest_field_with_last_tie_slash(("path/1/12", "x"))` gives `2`.
- `longest_field_with_last_tie_slash(("path/123", "z/w"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-476/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-476`.

[Module reading](https://doc.rust-lang.org/book/ch04-03-slices.html)
