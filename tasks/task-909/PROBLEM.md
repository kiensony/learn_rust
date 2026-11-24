# task-909 — Call a supplied closure · prefix before final mark (:)

Treat ':' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn prefix_before_final_mark_colon(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `prefix_before_final_mark_colon("hi:kit", |s| s.chars().count() + 1)` gives `3`.
- `prefix_before_final_mark_colon(":é:a", |s| s.len() * 2)` gives `6`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-909/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-909`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
