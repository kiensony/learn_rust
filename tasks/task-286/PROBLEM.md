# task-286 — Blank notice · tildes

Return '~' if text is empty after trimming whitespace. Otherwise return trimmed text.

```rust
pub fn blank_notice_tilde(text: &str) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `blank_notice_tilde("cat")` gives `"cat"`.
- `blank_notice_tilde("~hi~")` gives `"~hi~"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-286/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-286`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
