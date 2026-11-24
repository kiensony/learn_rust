# task-485 — Split a slice · first nonempty field (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Inspect the longer slice in the pair (first on a tie). Return the selected field’s byte length.

```rust
pub fn first_nonempty_field_pipe(fields: (&str, &str)) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `first_nonempty_field_pipe(("first\n123", "x"))` gives `3`.
- `first_nonempty_field_pipe(("hi|kit", "x"))` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-485/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-485`.

[Module reading](https://doc.rust-lang.org/book/ch04-03-slices.html)
