# task-925 — Call a supplied closure · leading marks removed (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn leading_marks_removed_pipe(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_marks_removed_pipe("first\n123", |s| s.chars().count() + 1)` gives `4`.
- `leading_marks_removed_pipe("header\n猫|123", |s| s.len() * 2)` gives `14`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-925/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-925`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
