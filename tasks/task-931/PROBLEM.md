# task-931 — Call a supplied closure · longest field with last tie (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn longest_field_with_last_tie_pipe(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `longest_field_with_last_tie_pipe("first\n123", |s| s.chars().count() + 1)` gives `4`.
- `longest_field_with_last_tie_pipe("header\n猫|123", |s| s.len() * 2)` gives `6`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-931/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-931`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
