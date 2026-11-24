# task-483 — Split a slice · remainder after first mark (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Inspect the longer slice in the pair (first on a tie). Return the selected field’s byte length.

```rust
pub fn remainder_after_first_mark_pipe(fields: (&str, &str)) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_pipe(("first\n123", "x"))` gives `0`.
- `remainder_after_first_mark_pipe(("header\n猫|123", "z|w"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-483/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-483`.

[Module reading](https://doc.rust-lang.org/book/ch04-03-slices.html)
