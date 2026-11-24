# task-464 — Split a slice · prefix before final mark (:)

Treat ':' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Inspect the longer slice in the pair (first on a tie). Return the selected field’s byte length.

```rust
pub fn prefix_before_final_mark_colon(fields: (&str, &str)) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `prefix_before_final_mark_colon(("hi:kit", "x"))` gives `2`.
- `prefix_before_final_mark_colon((":é:a", "z:w"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-464/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-464`.

[Module reading](https://doc.rust-lang.org/book/ch04-03-slices.html)
