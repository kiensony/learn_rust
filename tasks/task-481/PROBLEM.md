# task-481 — Split a slice · closing marks removed (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Inspect the longer slice in the pair (first on a tie). Return the selected field’s byte length.

```rust
pub fn closing_marks_removed_pipe(fields: (&str, &str)) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `closing_marks_removed_pipe(("first\n123", "x"))` gives `3`.
- `closing_marks_removed_pipe(("header\n猫|123", "z|w"))` gives `7`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-481/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-481`.

[Module reading](https://doc.rust-lang.org/book/ch04-03-slices.html)
