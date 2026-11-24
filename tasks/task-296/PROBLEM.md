# task-296 — Blank notice · plus signs

Return '+' if text is empty after trimming whitespace. Otherwise return trimmed text.

```rust
pub fn blank_notice_plus(text: &str) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `blank_notice_plus("cat")` gives `"cat"`.
- `blank_notice_plus("+hi+")` gives `"+hi+"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-296/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-296`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
