# task-511 — Measure Unicode text · closing marks removed (:)

Treat ':' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. After selecting the field, count its Unicode scalar values. An emoji occupies one scalar value in the examples.

```rust
pub fn closing_marks_removed_colon(text: &str) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_marks_removed_colon("hi:kit")` gives `6`.
- `closing_marks_removed_colon(":é:a")` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-511/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-511`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
