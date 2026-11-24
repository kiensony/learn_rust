# task-919 — Call a supplied closure · prefix before final mark (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn prefix_before_final_mark_slash(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `prefix_before_final_mark_slash("path/1/12", |s| s.chars().count() + 1)` gives `2`.
- `prefix_before_final_mark_slash("path/123", |s| s.len() * 2)` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-919/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-919`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
