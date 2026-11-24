# task-513 — Measure Unicode text · remainder after first mark (:)

Treat ':' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. After selecting the field, count its Unicode scalar values. An emoji occupies one scalar value in the examples.

```rust
pub fn remainder_after_first_mark_colon(text: &str) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_colon("hi:kit")` gives `3`.
- `remainder_after_first_mark_colon("")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-513/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-513`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
