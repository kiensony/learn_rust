# task-463 — Split a slice · remainder after first mark (:)

Treat ':' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Inspect the longer slice in the pair (first on a tie). Return the selected field’s byte length.

```rust
pub fn remainder_after_first_mark_colon(fields: (&str, &str)) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_colon(("hi:kit", "x"))` gives `3`.
- `remainder_after_first_mark_colon((":é:a", "z:w"))` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-463/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-463`.

[Module reading](https://doc.rust-lang.org/book/ch04-03-slices.html)
