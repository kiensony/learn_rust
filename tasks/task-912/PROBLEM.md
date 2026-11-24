# task-912 — Call a supplied closure · shortest field with first tie (:)

Treat ':' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn shortest_field_with_first_tie_colon(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shortest_field_with_first_tie_colon("hi:kit", |s| s.chars().count() + 1)` gives `3`.
- `shortest_field_with_first_tie_colon(":é:a", |s| s.len() * 2)` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-912/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-912`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
