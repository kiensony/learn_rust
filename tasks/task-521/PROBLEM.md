# task-521 — Measure Unicode text · closing marks removed (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. After selecting the field, count its Unicode scalar values. An emoji occupies one scalar value in the examples.

```rust
pub fn closing_marks_removed_slash(text: &str) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_marks_removed_slash("path/1/12")` gives `4`.
- `closing_marks_removed_slash("path/123")` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-521/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-521`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
