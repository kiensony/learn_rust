# task-276 — Blank notice · stars

Return '*' if text is empty after trimming whitespace. Otherwise return trimmed text.

```rust
pub fn blank_notice_star(text: &str) -> String
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `blank_notice_star("cat")` gives `"cat"`.
- `blank_notice_star("*hi*")` gives `"*hi*"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-276/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-276`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
