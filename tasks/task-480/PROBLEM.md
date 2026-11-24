# task-480 — Split a slice · leading marks removed (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Inspect the longer slice in the pair (first on a tie). Return the selected field’s byte length.

```rust
pub fn leading_marks_removed_pipe(fields: (&str, &str)) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_marks_removed_pipe(("first\n123", "x"))` gives `3`.
- `leading_marks_removed_pipe(("header\n猫|123", "z|w"))` gives `7`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-480/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-480`.

[Module reading](https://doc.rust-lang.org/book/ch04-03-slices.html)
