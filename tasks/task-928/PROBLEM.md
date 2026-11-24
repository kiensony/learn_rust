# task-928 — Call a supplied closure · remainder after first mark (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn remainder_after_first_mark_pipe(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_pipe("first\n123", |s| s.chars().count() + 1)` gives `1`.
- `remainder_after_first_mark_pipe("header\n猫|123", |s| s.len() * 2)` gives `6`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-928/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-928`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
